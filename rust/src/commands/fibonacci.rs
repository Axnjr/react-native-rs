use eyre::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[cfg(feature = "ts-rs")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct FibonacciInput {
    pub n: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct FibonacciResult {
    pub value: u64,
    pub computation_time_ms: u64,
    pub n: u32,
}

pub async fn fibonacci(input: &FibonacciInput) -> Result<FibonacciResult> {
    let start = Instant::now();
    
    // Use iterative approach for better performance
    let value = if input.n == 0 {
        0
    } else if input.n == 1 {
        1
    } else {
        let mut prev = 0u64;
        let mut curr = 1u64;
        
        for _ in 2..=input.n {
            let next = prev.saturating_add(curr);
            prev = curr;
            curr = next;
        }
        
        curr
    };
    
    let computation_time_ms = start.elapsed().as_millis() as u64;
    
    Ok(FibonacciResult {
        value,
        computation_time_ms,
        n: input.n,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fibonacci() {
        let input = FibonacciInput { n: 10 };
        let result = fibonacci(&input).await.unwrap();
        assert_eq!(result.value, 55);
        assert_eq!(result.n, 10);
    }

    #[tokio::test]
    async fn test_fibonacci_edge_cases() {
        // Test n = 0
        let result = fibonacci(&FibonacciInput { n: 0 }).await.unwrap();
        assert_eq!(result.value, 0);

        // Test n = 1
        let result = fibonacci(&FibonacciInput { n: 1 }).await.unwrap();
        assert_eq!(result.value, 1);
    }
}
