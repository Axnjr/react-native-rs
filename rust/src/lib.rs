#![deny(clippy::disallowed_methods)]

use cmd::{execute_cmd, Command};
use eyre::WrapErr;
use lazy_static::lazy_static;
use panic_handler::handle_panic;
use serde::Serialize;
use std::sync::Mutex;
use std::{
    ffi::{c_char, CStr, CString},
    panic,
    sync::Arc,
};
use tokio::runtime::{Builder, Runtime};
use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing_collector::VecMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub mod cmd;
mod commands;
mod panic_handler;
mod tracing_collector;

lazy_static! {
    pub static ref RUNTIME: Arc<Runtime> =
        Arc::new(Builder::new_multi_thread().enable_all().build().unwrap());
}

#[derive(Serialize)]
pub struct CommandResult {
    pub res: Option<String>,
    pub error: Option<String>,
    pub panic: bool,
    pub panic_details: Option<PanicDetails>,
}

#[derive(Serialize)]
pub struct PanicDetails {
    pub cmd: String,
    pub msg: String,
}

/// The interface between the React Native and Rust worlds.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn rust_execute(raw_cmd: *const c_char) -> *const c_char {
    static SETUP_RAYON: std::sync::Once = std::sync::Once::new();
    SETUP_RAYON.call_once(|| {
        // Configure thread pool based on platform
        #[cfg(target_os = "android")]
        let num_threads = (num_cpus::get() - 1).max(1);
        #[cfg(not(target_os = "android"))]
        let num_threads = num_cpus::get();

        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    });

    static SETUP_TRACING: std::sync::Once = std::sync::Once::new();
    static MESSAGE_STORE: Mutex<Vec<String>> = Mutex::new(Vec::new());
    SETUP_TRACING.call_once(|| {
        let vec_make_writer = VecMakeWriter::new(&MESSAGE_STORE);

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_writer(vec_make_writer))
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .parse_lossy(""),
            )
            .init();
    });

    // Parse the command ahead of time, returning early with error message if invalid
    let cmd_str = unsafe { CStr::from_ptr(raw_cmd).to_str().unwrap() };
    let cmd = match parse_command(cmd_str) {
        Ok(cmd) => Arc::new(cmd),
        Err(err) => {
            let invalid_cmd_res = CommandResult {
                res: None,
                error: Some(err.to_string()),
                panic: false,
                panic_details: None,
            };
            error!("cmd error: {:#}", err);
            let invalid_cmd_res = serde_json::to_string(&invalid_cmd_res).unwrap();
            return CString::new(invalid_cmd_res.as_bytes()).unwrap().into_raw();
        }
    };

    let exec_cmd = cmd.clone();
    let exec_res = panic::catch_unwind(|| {
        RUNTIME.block_on(async move {
            let cmd_res = execute_cmd(exec_cmd, &MESSAGE_STORE).await;

            let cmd_res = match cmd_res {
                Ok(res) => CommandResult {
                    res: Some(res),
                    error: None,
                    panic: false,
                    panic_details: None,
                },
                // Handled errors
                Err(err) => CommandResult {
                    res: None,
                    error: Some(format!("{} -> {}", err, err.root_cause())),
                    panic: false,
                    panic_details: None,
                },
            };

            let res_str = serde_json::to_string(&cmd_res).unwrap();
            CString::new(res_str.as_bytes()).unwrap().into_raw()
        })
    });

    // Handle panics, if any
    match exec_res {
        Ok(res) => res,
        Err(err) => {
            let panic_res_str = handle_panic(err, cmd);
            CString::new(panic_res_str.as_bytes()).unwrap().into_raw()
        }
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn rust_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        };
    }
}

fn parse_command(cmd_str: &str) -> Result<Command, eyre::Error> {
    serde_json::from_str::<Command>(cmd_str).wrap_err(format!(
        "failed to extract Command from raw command string: {cmd_str}"
    ))
}
