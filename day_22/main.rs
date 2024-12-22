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
    pub fn new() -> Result<Self, std::io::Error> {
        let temp_dir = std::env::temp_dir();
        let random_number: u64 = {
            let start = SystemTime::now();
            let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            since_epoch.as_nanos() as u64
        };
        let file_name = format!("tempfile-{}.tmp", random_number);
        let file_path = temp_dir.join(file_name);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;
        Ok(Self {
            file_path,
            file,
            content: String::new(),
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.file.write_all(data)?;
        self.content = data.iter().map(|&c| c as char).collect::<String>();

        Ok(())
    }

    pub fn read_from_cache(&self) -> &str {
        &self.content[..self.content.len()]
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}

fn main() {
    let mut temp_file = TempFile::new().unwrap();
    let data = b"Hello, world!";
    temp_file.write(data).unwrap();
    println!("Content from cache: {}", temp_file.read_from_cache());
    println!("Content from file: {}", temp_file.read_to_string().unwrap());
}
