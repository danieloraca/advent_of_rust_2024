use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
    content: String,
}

impl TempFile {
    /// Creates a new temporary file.
    pub fn new() -> Result<Self, std::io::Error> {
        let temp_dir = std::env::temp_dir();

        // Generate a random number based on the current system time.
        let random_number: u64 = {
            let start = SystemTime::now();
            let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            since_epoch.as_nanos() as u64
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
            file_path,
            file,
            content: String::new(), // Initialize content as an empty string
        })
    }

    /// Writes data to the file and updates the content field.
    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.file.write_all(data)?;
        self.file.flush()?; // Ensure all data is written to the file.

        // Append to the `content` field instead of overwriting.
        self.content.push_str(&String::from_utf8_lossy(data));
        Ok(())
    }

    /// Returns the content as a string slice.
    pub fn read_from_cache(&self) -> &str {
        &self.content
    }

    /// Reads the entire file into a String (owned value) and updates the cache.
    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.read_to_string(&mut buf)?;

        // Update the `content` field to keep it consistent.
        self.content = buf.clone();
        Ok(buf)
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

    // Write data to the file
    temp_file.write(b"Hello, Santa!")?;
    temp_file.write(b" Deliver all the gifts!")?;

    // Read from the in-memory cache
    println!("Content from cache: {}", temp_file.read_from_cache());

    // Read from the file itself
    let content = temp_file.read_to_string()?;
    println!("Content from file: {}", content);

    Ok(())
}
