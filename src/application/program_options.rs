use std::path::PathBuf;

use clap::Parser;

/// Parameters for the program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Parameters {
    /// Destination file
    ///
    /// If it is unspecified, standard output is used.
    #[arg(short, long, required = false)]
    pub output: Option<PathBuf>,

    /// Directory to scan for files
    ///
    /// If it is unspecified, current working directory is used.
    #[arg(required = false)]
    pub directory: Option<PathBuf>,
}
