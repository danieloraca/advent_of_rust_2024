pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
// impl<'a>
// 2. Create a public associated function named `new()`
// that will take a reference to a vector of strings
//
// 3. Create a public method named `search`
// that accepts a string slice and finds it from the logs and
//    returns a vector of references to those logs.

impl<'a> LogQuery<'a> {
    // 2. Create a public associated function named `new()`
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    // 3. Create a public method named `search`
    pub fn search(&self, query: &str) -> Vec<&String> {
        self.logs.iter().filter(|log| log.contains(query)).collect()
    }
}

fn main() {
    // Sample logs
    let logs = vec![
        String::from("Error: Failed to connect to database"),
        String::from("Info: User logged in successfully"),
        String::from("Warning: Low disk space"),
        String::from("Error: Timeout while waiting for response"),
        String::from("Info: File uploaded successfully"),
    ];

    // Initialize LogQuery
    let log_query = LogQuery::new(&logs);

    // Search for "Error" logs
    let error_logs = log_query.search("Error");
    println!("Error Logs:");
    for log in error_logs {
        println!("{}", log);
    }

    // Search for "Info" logs
    let info_logs = log_query.search("Info");
    println!("\nInfo Logs:");
    for log in info_logs {
        println!("{}", log);
    }

    // Search for "Debug" logs (which don't exist)
    let debug_logs = log_query.search("Debug");
    println!("\nDebug Logs:");
    if debug_logs.is_empty() {
        println!("No debug logs found.");
    } else {
        for log in debug_logs {
            println!("{}", log);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_query_new() {
        let logs = vec![String::from("Test log")];
        let log_query = LogQuery::new(&logs);
        assert_eq!(log_query.logs.len(), 1);
        assert_eq!(log_query.logs[0], "Test log");
    }

    #[test]
    fn test_search_existing() {
        let logs = vec![
            String::from("Error: Something went wrong"),
            String::from("Info: Operation completed successfully"),
        ];
        let log_query = LogQuery::new(&logs);
        let results = log_query.search("Error");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "Error: Something went wrong");
    }

    #[test]
    fn test_search_non_existing() {
        let logs = vec![
            String::from("Error: Something went wrong"),
            String::from("Info: Operation completed successfully"),
        ];
        let log_query = LogQuery::new(&logs);
        let results = log_query.search("Debug");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_multiple_matches() {
        let logs = vec![
            String::from("Error: Something went wrong"),
            String::from("Error: Another error occurred"),
            String::from("Info: Operation completed successfully"),
        ];
        let log_query = LogQuery::new(&logs);
        let results = log_query.search("Error");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "Error: Something went wrong");
        assert_eq!(results[1], "Error: Another error occurred");
    }
}
