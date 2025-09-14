use crate::commands::{
    fibonacci::{fibonacci, FibonacciInput},
    hash::{hash_data, HashDataInput},
    image_processing::{process_image, ImageProcessingInput},
    prime::{find_primes, FindPrimesInput},
    sort::{sort_numbers, SortNumbersInput},
};
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[cfg(feature = "ts-rs")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
#[serde(tag = "cmd", content = "params", rename_all = "snake_case")]
pub enum Command {
    /// Get system logs
    Logs,
    /// Calculate fibonacci number
    Fibonacci(FibonacciInput),
    /// Hash data using SHA-256
    HashData(HashDataInput),
    /// Sort an array of numbers
    SortNumbers(SortNumbersInput),
    /// Find prime numbers up to a limit
    FindPrimes(FindPrimesInput),
    /// Process image with filters
    #[cfg(feature = "image-processing")]
    ProcessImage(ImageProcessingInput),
}

pub async fn execute_cmd(
    cmd: Arc<Command>,
    logs: &'static Mutex<Vec<String>>,
) -> Result<String, eyre::Error> {
    match &*cmd {
        Command::Logs => {
            let Ok(mut logs) = logs.lock() else {
                return parse_result(Vec::<String>::new());
            };

            let logs = std::mem::take(&mut *logs);
            parse_result(logs)
        }
        Command::Fibonacci(input) => parse_result(fibonacci(input).await?),
        Command::HashData(input) => parse_result(hash_data(input).await?),
        Command::SortNumbers(input) => parse_result(sort_numbers(input).await?),
        Command::FindPrimes(input) => parse_result(find_primes(input).await?),
        #[cfg(feature = "image-processing")]
        Command::ProcessImage(input) => parse_result(process_image(input).await?),
    }
}

fn parse_result(res: impl Serialize) -> Result<String, eyre::Error> {
    serde_json::to_string(&res).wrap_err("failed to serialize command execution result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fibonacci_cmd() {
        let cmd_str = r#"{"cmd": "fibonacci", "params": {"n": 10}}"#;
        match serde_json::from_str::<Command>(cmd_str) {
            Ok(Command::Fibonacci(FibonacciInput { n })) => {
                assert_eq!(n, 10);
            }
            Ok(_) => panic!("wrong command"),
            Err(err) => panic!("failed to parse command: {err}"),
        }
    }

    #[test]
    fn parse_hash_cmd() {
        let cmd_str = r#"{"cmd": "hash_data", "params": {"data": "hello world"}}"#;
        match serde_json::from_str::<Command>(cmd_str) {
            Ok(Command::HashData(HashDataInput { data })) => {
                assert_eq!(data, "hello world");
            }
            Ok(_) => panic!("wrong command"),
            Err(err) => panic!("failed to parse command: {err}"),
        }
    }
}
