// This file will be responsible for parsing the CLI arguments creating the configuration, and then dispatching the work to our core logic.
// This is the heart of the application's execution flow. It's where the program starts and ends. We will keep it clean and simple, with its main job being to orchestrate the other modules.


// Explanation of this file : 
// mod ...; : these lines at the top declare the existence of our other modules, making their code accessible withing main.rs.
// main() -> Exitcode: This is the modern , idiomatic way to write a main function in Rust. Instead of calling std::process::exit(), we return an Exitcode. This ensures that all resources are properly cleaned up(a process known as "stack unwinding") before the program exits. ExitCode::SUCCESS corresponds to exit code 0, and ExitCode::FAILURE corresponds to 1.
// Seperation of main and run: We delegate all the fallible logic to run function that returns our RddResult<()>. This allows us to use the ? operator freely inside run. The main function;s only job is to call run and translate it Ok or Err result into the appropriate Exitcode, printing any errorrs to stderr. This is a very common and robust pattern in Rust applications.
// Orchestration: The run function clearly shows the intended flow: 
    //1. Parse arguments(Cli::parse()).
    //2. Validate and process them(CopyConfig::from_args(args)?).
    //3. Execute the core logic(the Err(RddError::NotImplemented(...))) part, which will replace next).
// src/main.rs

// Declare all the modules we have created or will create. This makes their contents available to the 'main' functions.

// src/main.rs

// Declare all the modules we have created or will create.
// This makes their contents available to the `main` function.
mod cli;
mod config;
mod core;
mod error;
mod utils;

use crate::cli::{Cli, Command};
use crate::config::CopyConfig;
use crate::core::copy::run_singlethreaded_copy;
use crate::error::{RddError,RddResult};
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    // The `run` function contains the application's primary logic.
    // By putting it in a separate function, we can use the `?` operator
    // for cleaner error handling. The result of `run` is then handled
    // in `main` to set the process exit code.
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            // Print the user-friendly error message to stderr.
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

/// The main logic loop of the application.
fn run() -> RddResult<()> {
    // Parse command-line arguments into our `Cli` struct.
    // `clap` will handle invalid arguments and printing help messages.
    let cli = Cli::parse();

    // Match on the subcommand to dispatch to the correct logic.
    // This structure makes it easy to add new commands in the future.
    match cli.command {
        Command::Copy(args) => {
            // 1. Create a validated configuration from the raw arguments.
            //    The `?` operator will propagate any configuration errors.
            let config = CopyConfig::from_args(args)?;

            // 2. Print a confirmation of the configuration for debugging.
            //    This will be replaced by the actual copy logic.
            println!("Starting copy from '{}' to '{}' with block size {} bytes.", config.input_file, config.output_file, config.block_size);

            // 3. Call the core copy function. The '?' operator will handle any I/O errors that occur.
            run_singlethreaded_copy(&config)?;
        }
    }
    Ok(())
}
