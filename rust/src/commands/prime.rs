use eyre::Result;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[cfg(feature = "ts-rs")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct FindPrimesInput {
    pub limit: u32,
    #[serde(default = "default_use_parallel")]
    pub use_parallel: bool,
}

fn default_use_parallel() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct FindPrimesResult {
    pub primes: Vec<u32>,
    pub count: usize,
    pub limit: u32,
    pub computation_time_ms: u64,
    pub used_parallel: bool,
}

pub async fn find_primes(input: &FindPrimesInput) -> Result<FindPrimesResult> {
    let start = Instant::now();
    
    let primes = if input.use_parallel {
        find_primes_parallel(input.limit)
    } else {
        find_primes_sequential(input.limit)
    };
    
    let computation_time_ms = start.elapsed().as_millis() as u64;
    let count = primes.len();
    
    Ok(FindPrimesResult {
        primes,
        count,
        limit: input.limit,
        computation_time_ms,
        used_parallel: input.use_parallel,
    })
}

fn find_primes_sequential(limit: u32) -> Vec<u32> {
    if limit < 2 {
        return vec![];
    }
    
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as u32) {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    
    (2..=limit)
        .filter(|&i| is_prime[i as usize])
        .collect()
}

fn find_primes_parallel(limit: u32) -> Vec<u32> {
    if limit < 2 {
        return vec![];
    }
    
    // For small limits, sequential is faster due to overhead
    if limit < 1000 {
        return find_primes_sequential(limit);
    }
    
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    
    // Sequential sieve for small primes
    for i in 2..=((limit as f64).sqrt() as u32) {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    
    // Parallel collection of results
    (2..=limit)
        .into_par_iter()
        .filter(|&i| is_prime[i as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_primes() {
        let input = FindPrimesInput {
            limit: 20,
            use_parallel: false,
        };
        
        let result = find_primes(&input).await.unwrap();
        assert_eq!(result.primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(result.count, 8);
    }

    #[tokio::test]
    async fn test_find_primes_parallel() {
        let input = FindPrimesInput {
            limit: 20,
            use_parallel: true,
        };
        
        let result = find_primes(&input).await.unwrap();
        let mut expected = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let mut actual = result.primes.clone();
        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);
        assert_eq!(result.count, 8);
    }

    #[tokio::test]
    async fn test_find_primes_edge_cases() {
        // Test limit = 0
        let result = find_primes(&FindPrimesInput { limit: 0, use_parallel: false }).await.unwrap();
        assert_eq!(result.primes, Vec::<u32>::new());
        
        // Test limit = 1
        let result = find_primes(&FindPrimesInput { limit: 1, use_parallel: false }).await.unwrap();
        assert_eq!(result.primes, Vec::<u32>::new());
        
        // Test limit = 2
        let result = find_primes(&FindPrimesInput { limit: 2, use_parallel: false }).await.unwrap();
        assert_eq!(result.primes, vec![2]);
    }
}
