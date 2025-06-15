// Explanation of this file:
// use thiserror::Error; -> We import the Error derive macro from the thiserror crate we added to Cargo.toml.
// #[derive(Error, Debug)] -> This is the magic of thiserror. It automatically implements the
// standard std::error::Error trait for our RddError enum, saving us a lot of boilerplate code. Debug allows the error to be printed for debugging purposes.
// #[error("...")] -> This attribute provides the user-facing error message for each variant of our enum.
// #[from] io::Error -> This is a powerful helper. It tells Rust how to automatically convert a standard std::io::Error into our RddError::Io variant. This means if we have a function that does file I/O and returns a std::io::Result, we can use the ? operator on it, and the error will be seamlessly converted into our application's error type.
// RddResult<T> : This is a command Rust idiom. We create a type alias for REsult<T, RddError>. This makes function signatures much cleaner throughout our project.

// This file defines a single. comprehensive error type for our entire program

// src/error.rs

use std::io;
use thiserror::Error;

/// The unified error type for all fallible operations in 'rdd'.
/// This enum is designed to provide clear, user-friendly error messagesfor every potential failure point in the application, from IO problems to configuration mistakes.

#[derive(Error, Debug)]
pub enum RddError {

    /// Error originating from standard I/O operations (e.g., reading a file, writing to a device).
    /// This '#[from]' attribute allows for automatic conversion from 'std::io::Error.
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    /// Error during the parsing of command-line arguments or configuration values, such as an invalid block size string.
    #[error("Configuration Error: {0}")]
    Config(String),

    /// Error when a data verification hash check fails. This provides clear feedback on why the verification did not succeed.
    #[error("Verification failed: Hashes do not match. Expected: {expected}, Got: {actual}")]
    VerificationFailure { expected: String, actual: String },

    /// Error when a multithreading channel operation fails, indicating a breakdown in communication between the reader and writer threads.
    #[error("Threading channel error: {0}")]
    Channel(String), 

    /// A placeholder for features that are planned but not yet implemented. Useful for scaffolding the CLI and logic.
    #[error("Features not yet implement: {0}")]
    NotImplemented(String),
}

/// A specialized 'Result' type for 'rdd operations. Using this alias simplifies function signatures throughout the crate, making the code cleaners and more readable. Instead of 'Result<T, RddError', we can just write RddResult<T>'.
pub type RddResult<T> = Result<T, RddError>;

