#[cfg(feature = "ts-rs")]
fn main() {
    use react_native_rs::cmd::Command;
    use react_native_rs::commands::{
        fibonacci::{FibonacciInput, FibonacciResult},
        hash::{HashDataInput, HashDataResult, HashAlgorithm},
        prime::{FindPrimesInput, FindPrimesResult},
        sort::{SortNumbersInput, SortNumbersResult, SortAlgorithm},
    };
    
    #[cfg(feature = "image-processing")]
    use react_native_rs::commands::image_processing::{
        ImageProcessingInput, ImageProcessingResult, ImageFilter
    };
    
    println!("Generating TypeScript types...");
    
    // Export all types
    Command::export().expect("Failed to export Command");
    
    FibonacciInput::export().expect("Failed to export FibonacciInput");
    FibonacciResult::export().expect("Failed to export FibonacciResult");
    
    HashDataInput::export().expect("Failed to export HashDataInput");
    HashDataResult::export().expect("Failed to export HashDataResult");
    HashAlgorithm::export().expect("Failed to export HashAlgorithm");
    
    SortNumbersInput::export().expect("Failed to export SortNumbersInput");
    SortNumbersResult::export().expect("Failed to export SortNumbersResult");
    SortAlgorithm::export().expect("Failed to export SortAlgorithm");
    
    FindPrimesInput::export().expect("Failed to export FindPrimesInput");
    FindPrimesResult::export().expect("Failed to export FindPrimesResult");
    
    #[cfg(feature = "image-processing")]
    {
        ImageProcessingInput::export().expect("Failed to export ImageProcessingInput");
        ImageProcessingResult::export().expect("Failed to export ImageProcessingResult");
        ImageFilter::export().expect("Failed to export ImageFilter");
    }
    
    println!("TypeScript types generated successfully!");
}

#[cfg(not(feature = "ts-rs"))]
fn main() {
    println!("TypeScript generation requires the 'ts-rs' feature to be enabled.");
    println!("Run with: cargo run --bin generate-types --features ts-rs");
}
