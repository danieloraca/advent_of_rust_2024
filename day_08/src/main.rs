use std::fs::File;
use std::io::Write;

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁
        // Search for logs containing the keyword
        let matching_logs = self.search(keyword);

        // Open the file in write mode (this creates the file or truncates it if it exists)
        let mut file = File::create(path)?;

        // Write each matching log to the file, separated by newlines
        for log in matching_logs {
            writeln!(file, "{}", log)?;
        }

        Ok(())
    }
}

fn main() {
    let logs = vec![
        "2024-01-01 12:00:00 [INFO] User logged in".to_string(),
        "2024-01-01 12:01:00 [INFO] User logged out".to_string(),
        "2024-01-01 12:02:00 [ERROR] Invalid password".to_string(),
    ];

    let log_query = LogQuery::new(&logs);
    log_query
        .export_to_file("[ERROR]", "error_logs.txt")
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_search() {
        let logs = vec![
            "2024-01-01 12:00:00 [INFO] User logged in".to_string(),
            "2024-01-01 12:01:00 [INFO] User logged out".to_string(),
            "2024-01-01 12:02:00 [ERROR] Invalid password".to_string(),
        ];

        let log_query = LogQuery::new(&logs);
        let matching_logs = log_query.search("[ERROR]");

        assert_eq!(matching_logs.len(), 1);
        assert_eq!(matching_logs[0], &logs[2]);
    }

    #[test]
    fn test_export_to_file() {
        let logs = vec![
            "2024-01-01 12:00:00 [INFO] User logged in".to_string(),
            "2024-01-01 12:01:00 [INFO] User logged out".to_string(),
            "2024-01-01 12:02:00 [ERROR] Invalid password".to_string(),
        ];

        let log_query = LogQuery::new(&logs);
        log_query
            .export_to_file("[ERROR]", "error_logs.txt")
            .unwrap();

        let file_contents = fs::read_to_string("error_logs.txt").unwrap();
        assert_eq!(
            file_contents,
            "2024-01-01 12:02:00 [ERROR] Invalid password\n"
        );
    }
}
