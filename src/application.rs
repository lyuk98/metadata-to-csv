use std::{
    env::current_dir,
    error::Error,
    fs::{read_dir, File},
    io::{stdin, stdout, ErrorKind, Write},
};

use clap::Parser;
use csv::Writer;
use program_options::Parameters;

mod file;
mod program_options;

/// Run the application
pub fn run() -> Result<(), Box<dyn Error>> {
    // Parse parameters
    let parameters = Parameters::parse();

    // Create output writer
    let output = match parameters.output {
        // Try creating and opening a new file
        Some(ref path) => match File::create_new(path) {
            // Return the file handle upon success
            Ok(file) => Box::new(file),
            Err(error) => match error.kind() {
                // Prompt the user in case the file exists
                ErrorKind::AlreadyExists => {
                    eprint!("'{}' already exists. Overwrite? ", path.display());

                    // Ask for input
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;

                    match input.to_lowercase().trim() == "y" {
                        // Truncate existing file in case the user
                        // answered "y"
                        true => Box::new(File::create(path)?),
                        // Abort if the user decided not to overwrite
                        false => return Err(From::from(error)),
                    }
                }
                // Abort for other errors
                _ => return Err(From::from(error)),
            },
        },
        // Use stdout as output when no file path was provided
        None => Box::new(stdout()) as Box<dyn Write>,
    };

    // List files in the provided directory
    let mut directory = match parameters.directory {
        // Try reading the provided directory
        Some(ref path) => read_dir(path)?,
        // Try using the current working directory
        None => read_dir(current_dir()?)?,
    };

    // Read directory and save information
    let data = file::read_directory(&mut directory)?;

    // Build a CSV writer using previously defined output stream
    let mut writer = Writer::from_writer(output);

    // Write data into file
    writer.write_record(&["File name", "File size", "Timestamp"])?;
    for entry in data {
        writer.write_record(&[
            entry.file_name.to_string_lossy().into(),
            entry.metadata.len().to_string(),
            entry.timestamp.format("%F %T").to_string(),
        ])?;
    }

    // Flush the writer
    writer.flush()?;

    Ok(())
}
