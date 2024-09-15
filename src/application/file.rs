use std::{
    cmp::Ordering,
    error::Error,
    ffi::OsString,
    fs::{DirEntry, File, Metadata, ReadDir},
};

use chrono::{DateTime, FixedOffset, Local};
use nom_exif::{parse_exif, Exif, ExifTag};

/// Information of a file
pub struct FileInformation {
    /// File name
    pub file_name: OsString,
    /// Metadata
    pub metadata: Metadata,
    /// Timestamp
    pub timestamp: DateTime<FixedOffset>,
}

impl Ord for FileInformation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            Ordering::Equal => self.file_name.cmp(&other.file_name),
            others => others,
        }
    }
}
impl PartialOrd for FileInformation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for FileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp && self.file_name == other.file_name
    }
}
impl Eq for FileInformation {}

/// Read and return information about a file
///
/// The function reads the file using path provided by the file entry
/// and attempts to read EXIF data, retrieving the time the photo was
/// taken. If it fails, modified time of the file in the file system is
/// used instead.
fn read_file(file: &DirEntry) -> Result<Option<FileInformation>, Box<dyn Error>> {
    // Proceed only if the entry points to a regular file
    if !file.file_type()?.is_file() {
        return Ok(None);
    }

    // Get metadata
    let metadata = file.metadata()?;
    // Retrieve timestamp
    let timestamp = match match File::open(file.path()) {
        // Try opening the file and parsing EXIF data
        Ok(file) => match parse_exif(file, None) {
            Ok(iter) => match iter {
                // Get the time the photo was taken
                Some(iter) => {
                    let exif: Exif = iter.into();
                    match exif.get(ExifTag::DateTimeOriginal) {
                        Some(datetime) => datetime.as_time(),
                        None => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        },
        Err(_) => None,
    } {
        // Use the EXIF timestamp if it exists
        Some(datetime) => datetime,
        // Get modified time from file system otherwise
        None => {
            let datetime_local: DateTime<Local> = metadata.modified()?.into();
            datetime_local.into()
        }
    };

    // Return the data
    Ok(Some(FileInformation {
        file_name: file.file_name(),
        metadata,
        timestamp,
    }))
}

/// Read and return information about files in a directory
///
/// The function reads the files using provided ReadDir instance and
/// attempts to read EXIF data, retrieving the time photos were taken.
/// For files that such operation fails, modified time of them in the
/// file system is used instead.
pub fn read_directory(directory: &mut ReadDir) -> Result<Vec<FileInformation>, Box<dyn Error>> {
    // Store the result in a Vec
    let mut files: Vec<FileInformation> = Vec::new();

    // Iterate through the directory
    for entry in directory {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => return Err(From::from(error)),
        };

        // Call read_file() to get information
        match read_file(&entry)? {
            // Push to Vec only if the result contains information
            Some(info) => files.push(info),
            None => (),
        };
    }

    // Sort the Vec by timestamp
    files.sort_unstable();

    // Return the data
    Ok(files)
}
