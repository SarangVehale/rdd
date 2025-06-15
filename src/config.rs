//This file will contain a CopyConfig struct that holds the final, validated settings for a copy operation. It will also include the logic to parse human-readable size strings like "4k" or "128M" into a precise number of bytes.

// Explanation of this file:
// CopyConfig Struct: This is our clean, internal representation of the job to be done. Notice that block_size is a usize, the correct type for memory allocations and buffer size in Rust.
// from_args Function: This acts as a bridge between the cli module and our application logic. It takes the CopyArgs struct (which contains strings) and produces a validated CopyConfig struct (which contains correctly typed data).
// parse_size Function: This is the workhorse of the module
    // It's robust : It handles whitespace, is case-insensitive, and provides clear error messages for invalid numbers or suffixes.
    // It's safe : It uses checked_mul to prevent integer overflows if a user specifies an enormous number (e.g., 1000000T)
    // It's architecture-aware: It uses usize::try_from to ensure the final size fits into the memory space of the target machine (a u64 can be larger than a usize on a 32-bit system.)
// Validation: We added a check to ensure block_size is not zero, which would cause an infinite loop or a panic in the copy logic. This is the kind of validation this module is reponsible for.

// src/config.rs

use crate::cli::{CopyArgs, HashAlgorithm};
use crate::error::{RddError, RddResult};

/// A validated and processed configuration for a copy operation.
///
/// This struct holds all the necessary parameters for the core copy logic,
/// with data types that are ready for immediate use (e.g., `block_size` is a
/// `usize`, not a `String`).
#[derive(Debug)]
pub struct CopyConfig {
    pub input_file: String,
    pub output_file: String,
    pub block_size: usize,
    pub count: u64,
    pub skip: u64,
    pub seek: u64,
    pub show_progress: bool,
    pub verification_algo: Option<HashAlgorithm>,
    pub threads: u8,
    #[cfg(unix)]
    pub use_direct_io: bool,
}

impl CopyConfig {
    /// Creates a new `CopyConfig` from the raw command-line arguments.
    ///
    /// This function is responsible for parsing and validating the arguments
    /// provided by the user.
    pub fn from_args(args: CopyArgs) -> RddResult<Self> {
        let block_size = parse_size(&args.bs)?;

        // The block size must not be zero.
        if block_size == 0 {
            return Err(RddError::Config("Block size cannot be zero.".to_string()));
        }

        Ok(Self {
            input_file: args.input,
            output_file: args.output,
            block_size,
            count: args.count,
            skip: args.skip,
            seek: args.seek,
            show_progress: args.progress,
            verification_algo: args.verify,
            threads: args.threads,
            #[cfg(unix)]
            use_direct_io: args.direct,
        })
    }
}

/// Parses a size string (e.g., "512k", "1M", "2G") into a number of bytes.
///
/// This function is case-insensitive and supports standard suffixes.
fn parse_size(s: &str) -> RddResult<usize> {
    let s_trimmed = s.trim();
    if s_trimmed.is_empty() {
        return Err(RddError::Config("Size string cannot be empty.".to_string()));
    }

    let s_lower = s_trimmed.to_lowercase();

    let (num_str, suffix) = s_lower
        .find(|c: char| !c.is_ascii_digit())
        .map(|i| s_lower.split_at(i))
        .unwrap_or((&s_lower, ""));

    let num = num_str.parse::<u64>().map_err(|_| {
        RddError::Config(format!("Invalid numeric value in size: '{}'", s_trimmed))
    })?;

    let multiplier = match suffix {
        "" => 1, // No suffix means bytes
        "k" | "kb" => 1024,
        "m" | "mb" => 1024 * 1024,
        "g" | "gb" => 1024 * 1024 * 1024,
        "t" | "tb" => 1024 * 1024 * 1024 * 1024,
        _ => {
            return Err(RddError::Config(format!(
                "Unknown size suffix: '{}'",
                suffix
            )))
        }
    };

    let final_size = num.checked_mul(multiplier).ok_or_else(|| {
        RddError::Config(format!(
            "Size value '{}' is too large and would overflow.",
            s_trimmed
        ))
    })?;

    usize::try_from(final_size).map_err(|_| {
        RddError::Config(format!(
            "Size value '{}' is too large for this system's architecture (max is {} bytes).",
            s_trimmed,
            usize::MAX
        ))
    })
}
