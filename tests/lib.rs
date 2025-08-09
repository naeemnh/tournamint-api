// Test library configuration
// This file enables running unit tests from the tests directory

// Import the main server library for testing
extern crate server;

// Import unit test modules
pub mod unit;

// Test configuration
#[cfg(test)]
mod tests {
    use super::*;
    
    // This ensures the test module is properly compiled
    #[test]
    fn test_modules_compile() {
        // This test just ensures all modules compile correctly
        assert!(true);
    }
}