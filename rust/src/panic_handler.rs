use crate::{cmd::Command, CommandResult, PanicDetails};
use std::sync::Arc;
use tracing::error;

pub fn handle_panic(err: Box<dyn std::any::Any + Send>, cmd: Arc<Command>) -> String {
    let panic_msg = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic".to_string()
    };

    let cmd_name = format!("{:?}", cmd).split('(').next().unwrap_or("Unknown").to_string();
    
    error!("Panic in command {}: {}", cmd_name, panic_msg);

    let panic_result = CommandResult {
        res: None,
        error: None,
        panic: true,
        panic_details: Some(PanicDetails {
            cmd: cmd_name,
            msg: panic_msg,
        }),
    };

    serde_json::to_string(&panic_result).unwrap_or_else(|_| {
        r#"{"res":null,"error":"Failed to serialize panic result","panic":true,"panic_details":null}"#.to_string()
    })
}
