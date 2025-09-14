use eyre::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Instant;

#[cfg(feature = "ts-rs")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct HashDataInput {
    pub data: String,
    #[serde(default = "default_algorithm")]
    pub algorithm: HashAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub enum HashAlgorithm {
    #[serde(rename = "sha256")]
    Sha256,
}

fn default_algorithm() -> HashAlgorithm {
    HashAlgorithm::Sha256
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct HashDataResult {
    pub hash: String,
    pub algorithm: HashAlgorithm,
    pub input_length: usize,
    pub computation_time_ms: u64,
}

pub async fn hash_data(input: &HashDataInput) -> Result<HashDataResult> {
    let start = Instant::now();
    
    let hash = match input.algorithm {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(input.data.as_bytes());
            format!("{:x}", hasher.finalize())
        }
    };
    
    let computation_time_ms = start.elapsed().as_millis() as u64;
    
    Ok(HashDataResult {
        hash,
        algorithm: input.algorithm.clone(),
        input_length: input.data.len(),
        computation_time_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_data() {
        let input = HashDataInput {
            data: "hello world".to_string(),
            algorithm: HashAlgorithm::Sha256,
        };
        
        let result = hash_data(&input).await.unwrap();
        assert_eq!(
            result.hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        assert_eq!(result.input_length, 11);
    }
}
