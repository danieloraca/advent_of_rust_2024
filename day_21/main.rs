use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct TempFile {
    file_path: PathBuf,
    file: File,
}
impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut file_path = PathBuf::new();
        file_path.push(std::env::temp_dir());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        file_path.push(format!("{}", timestamp));
        let file = File::create(&file_path)?;
        Ok(Self { file_path, file })
    }
    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(self.path())?;
        file.write_all(data)
    }
    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(self.path())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
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
        let _ = std::fs::remove_file(self.path());
    }
}
