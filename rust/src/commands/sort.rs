use eyre::Result;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[cfg(feature = "ts-rs")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct SortNumbersInput {
    pub numbers: Vec<i32>,
    #[serde(default = "default_algorithm")]
    pub algorithm: SortAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub enum SortAlgorithm {
    #[serde(rename = "parallel")]
    Parallel,
    #[serde(rename = "sequential")]
    Sequential,
}

fn default_algorithm() -> SortAlgorithm {
    SortAlgorithm::Parallel
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct SortNumbersResult {
    pub sorted_numbers: Vec<i32>,
    pub algorithm: SortAlgorithm,
    pub input_length: usize,
    pub computation_time_ms: u64,
}

pub async fn sort_numbers(input: &SortNumbersInput) -> Result<SortNumbersResult> {
    let start = Instant::now();
    let mut numbers = input.numbers.clone();
    
    match input.algorithm {
        SortAlgorithm::Parallel => {
            numbers.par_sort_unstable();
        }
        SortAlgorithm::Sequential => {
            numbers.sort_unstable();
        }
    }
    
    let computation_time_ms = start.elapsed().as_millis() as u64;
    
    Ok(SortNumbersResult {
        sorted_numbers: numbers,
        algorithm: input.algorithm.clone(),
        input_length: input.numbers.len(),
        computation_time_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sort_numbers() {
        let input = SortNumbersInput {
            numbers: vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            algorithm: SortAlgorithm::Parallel,
        };
        
        let result = sort_numbers(&input).await.unwrap();
        assert_eq!(result.sorted_numbers, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
        assert_eq!(result.input_length, 11);
    }

    #[tokio::test]
    async fn test_sort_empty() {
        let input = SortNumbersInput {
            numbers: vec![],
            algorithm: SortAlgorithm::Sequential,
        };
        
        let result = sort_numbers(&input).await.unwrap();
        assert_eq!(result.sorted_numbers, Vec::<i32>::new());
        assert_eq!(result.input_length, 0);
    }
}
