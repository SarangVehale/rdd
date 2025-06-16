// This file will contain the definitions for our application''s commands, arguments, and flags. By using clap's "derive" feature, we can define the entire CLI structure using simple, commented Rust structs. This makes the code self-documenting and easy to maintain.

// ===================================================================================

// Explanation of this file : 

// #[derive(Parser)]: this is the main macro from clap. It instructs clap to generate all the command-line parsing logic based on the fields of the struct.
// Doc Comments (///) : the triple slash comments are special. clap uses them to automatically generate the help messages for your application. What you write here is what the user will see when they run rdd --help.
// #[]command(...)] : this attribute provides top-level information about your application, like the author and a longer description.
// Subcommands : The Cli and Command enums create a subcommand structure (e.g., rdd copy...). This is a modern CLI pattern that makes the tool extensible. For now, we only have the copy subcommand.
// #[arg(...)] : This attribute configures each command-line argument.
    // long : Defines the long name (e.g., --input)
    // short : Defines the optional short name(e.g., -i)
    // required = true : Makes the argument mandatory.
    // default_value = "..." : Provides a default value if the user doesn't specify one.
    // value_enum : Used for the --verify flag to restrict its value to one of the HashAlgorithm enum variants.
//#[cfg(unix)] : This is a conditional compilation attribute. The --direct flag will only exist if
//the program is compiled on a Unix-like system (Linux, macOS). This prevents compilation errors on Windows.

// src/cli.rs

use clap::{Parser, Subcommand, ValueEnum};

/// rdd: A  modern, safe, and fast replacement for GNU dd.

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "rdd is a utility for copying and converting data. It replicates the core functionality of dd while adding modern features like rich progress bars, multithreading, and on-the-fly hash verification."
    )]

pub struct Cli{
    #[command(subcommand) ]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// The core disk/file copy operation, mirroring dd's functionality.
    Copy(CopyArgs),
    // Future subcommands like 'verify' or 'partition' would be added here.
}

/// Arguments for the 'copy' command
#[derive(Parser, Debug)]
pub struct CopyArgs {
    /// Input file or device (e.g., /dev/sda, image.iso).
    #[arg(long, short, value_name = "FILE", required = true)]
    pub input: String,

    /// Output file or device.
    #[arg(long, short, value_name = "FILE", required = true)]
    pub output: String,

    /// Block size in bytes. Supports suffixes: k, M, G (e.g., 4k, 128M, 2G).
    #[arg(long, short = 'b', value_name = "SIZE", default_value = "512k")]
    pub bs: String,

    /// Number of blocks to copy (if 0, copies until end of input).
    #[arg(long, short, value_name = "N", default_value_t = 0)]
    pub count: u64,

    /// Skip N blocks of 'bs size at the start of the input.
    #[arg(long, default_value_t = 0)]
    pub skip: u64,
    
    /// Seek N blocks of 'bs' size at the start of the output.
    #[arg(long, default_value_t=0)]
    pub seek: u64,

    /// [Enhancement] Hashing algorithm to verify data integrity during the copy.
    #[arg(long, value_enum)]
    pub verify: Option<HashAlgorithm>,

    /// Show a rich progress bar and live statistics (enabled by default).
    #[arg(long, default_value_t=true, action = clap::ArgAction::SetTrue)]
    pub progress: bool,

    /// [Enhancement] Number of threads for I/O (1=single=threaded, >1 = multithreaded).
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
    pub threads: u8,

    /// [Unix-only] Use O_DIRECT to bypass the OS page cache for I/O. This can improve performance for large transfers on fast devices but may degrade it in other cases. Requires block size to be aligned to the filesystem's logical block size.
    #[cfg(unix)]
    #[arg(long)]
    pub direct: bool,
}

/// Supported hashing algorithms for the --verfiy flag.
#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha256,
    Blake3
}
