use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    file: File,
    file_path: PathBuf,
    content: Vec<u8>, // Use Vec<u8> to store raw file content
}

impl TempFile {
    /// Creates a new temporary file.
    pub fn new() -> Result<Self, std::io::Error> {
        let temp_dir = std::env::temp_dir();

        // Generate a random number based on the current system time.
        let random_number = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => 0,
        };

        let file_name = format!("tempfile-{}.tmp", random_number);
        let file_path = temp_dir.join(file_name);

        // Open the file with read/write/create options.
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;

        Ok(Self {
            file,
            file_path,
            content: Vec::new(), // Initialize content as an empty Vec<u8>
        })
    }

    /// Writes data to the file and updates the content field.
    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.file.write_all(data)?;
        self.file.flush()?; // Ensure all data is written to the file.

        // Update the `content` field with the written data.
        self.content = data.to_vec();
        Ok(())
    }

    /// Returns the content as a string slice.
    pub fn read_from_cache(&self) -> &str {
        // Convert Vec<u8> to &str using std::str::from_utf8.
        match std::str::from_utf8(&self.content) {
            Ok(s) => s,
            Err(_) => "",
        }
    }

    /// Reads the entire file into a String (owned value) and updates the cache.
    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut buf = Vec::new();
        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.read_to_end(&mut buf)?;

        // Update the `content` field with the data read from the file.
        self.content = buf.clone();

        // Convert the bytes to a String and return.
        Ok(match String::from_utf8(buf) {
            Ok(s) => s,
            Err(_) => String::new(),
        })
    }

    /// Returns the file path.
    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    /// Returns a reference to the file.
    pub fn file(&self) -> &File {
        &self.file
    }
}

/// Automatically deletes the temporary file when the struct goes out of scope.
impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut temp_file = TempFile::new()?;

    // Write some data to the file.
    temp_file.write(b"Hello, World!")?;

    // Read from the cache.
    println!("Content from cache: {}", temp_file.read_from_cache());

    // Read from the file itself.
    let content = temp_file.read_to_string()?;
    println!("Content from file: {}", content);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_file() {
        let mut temp_file = TempFile::new().unwrap();
        let data = b"Hello, world!";
        temp_file.write(data).unwrap();
        assert_eq!(temp_file.read_from_cache(), "Hello, world!");
        assert_eq!(temp_file.read_to_string().unwrap(), "Hello, world!");
    }
}
