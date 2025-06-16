// This is kinda the most important file in our project. It will contain the single-threaded dd replication logic. It will open the input and output files, seek to the correct positions, and then enter a loop to read blocks from the source and write them to destination.

// Explanation of this file:
// Function signature: It takes a reference to our CopyConfig struct, which contains all the necesary parameters. It returns our RddResult<()>, so it can signal success (Ok(())) or failure (Err(RddError)).
// File handling: It uses std::fs::File to open the input and std::fs::OpenOptions to gain more control over how the output file is opened (write, create, truncate).
// skip and seek: It uses the seek method on the file handles to move the read/ write cursors to the correct starting position before the loop begins. This is a direct implementation of dd's skip and seek operands.
// The Buffer: let mut buffer = vec![0;config.block_size]; creates a block of memory on the heap that we will reuse for every read/write cycle. This is efficient.
// The Loop:
    // It first checks the count condition.
    // input_file.read(&mut buffer)? attempts to fill the entire buffer from the input file. It returns the number of bytes actually read.
    // if bytes_read == 0: this is the standard way to detect the end of a file (EOF) when reading.
    // output_file.write_all(&buffer [..bytes_read])?: This is the most critical line. We write only the bytes that were read. If we wrote the whole buffer, we would write garbage data on the last, partial block. 
    // output_file.sync_all()?: this is crucial for data integrity. It tells the operating system to flush all its internal write caches to the physical disk. This ensures that when rdd exits, the data is safely stored. IT's the equivalent of dd's conv=fsync.

// src/core/copy.rs

// src/core/copy.rs

use crate::config::CopyConfig;
use crate::error::RddResult;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

/// Runs the core copy operation in a single thread.
///
/// This function orchestrates the entire copy process: opening files, seeking to
/// the correct positions, and executing the main read/write loop.
pub fn run_singlethreaded_copy(config: &CopyConfig) -> RddResult<()> {
    // Open the input file for reading.
    let mut input_file = File::open(&config.input_file)?;

    // Open the output file for writing, creating it if it doesn't exist.
    // We truncate it by default, mimicking dd's behavior.
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config.output_file)?;

    // --- Handle seek/skip options ---
    // Move the cursor in the input file if `skip` is specified.
    if config.skip > 0 {
        let skip_bytes = config.skip * config.block_size as u64;
        input_file.seek(SeekFrom::Start(skip_bytes))?;
    }

    // Move the cursor in the output file if `seek` is specified.
    if config.seek > 0 {
        let seek_bytes = config.seek * config.block_size as u64;
        output_file.seek(SeekFrom::Start(seek_bytes))?;
    }

    // --- Main Copy Loop ---
    // Create a buffer with the specified block size.
    // Using `vec!` is fine, but `with_capacity` followed by `set_len` can be
    // slightly more performant for very large block sizes, though it requires `unsafe`.
    // For clarity and safety, `vec!` is preferred here.
    let mut buffer = vec![0; config.block_size];
    let mut blocks_copied = 0u64;

    loop {
        // Check if the `count` limit has been reached.
        if config.count > 0 && blocks_copied >= config.count {
            break;
        }

        // Read a block from the input file into the buffer.
        let bytes_read = input_file.read(&mut buffer)?;

        // If `read` returns 0, we've reached the end of the file.
        if bytes_read == 0 {
            break;
        }

        // Write the portion of the buffer that was filled to the output file.
        // It's crucial to use `&buffer[..bytes_read]` because the last block
        // may not be a full block.
        output_file.write_all(&buffer[..bytes_read])?;

        blocks_copied += 1;
    }

    // Ensure all buffered data is written to the disk before exiting.
    // This is equivalent to dd's `conv=fsync`.
    output_file.sync_all()?;

    println!(
        "{} blocks copied successfully.",
        blocks_copied
    );

    Ok(())
}
