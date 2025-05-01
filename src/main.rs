use std::{ fs::File, io::{ BufReader, Read }, path::Path };
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

/*
    # Rust Bytes Challenge - Issue #64 From Rust Bytes
    # Credit: Silen1t
    # Github: https://github.com/Silen1t
*/

#[derive(Serialize, Deserialize)]
struct LogEntry {
    user_id: String,
    action: String,
    timestamp: DateTime<Utc>,
}

fn main() {
    let log_path = Path::new("activity_log.json");
    let (count, _uniqe_users) = get_uniqe_users_from_file(log_path);
    println!("We have {count} uniqe user");
}

fn get_uniqe_users_from_file<'a>(log_path: &Path) -> (usize, Vec<String>) {
    let err_count: usize = 0;
    let mut uniqe_users: Vec<String> = Vec::new();

    match read_json_file(&log_path) {
        Ok(file_content) => {
            match sonic_rs::from_str::<Vec<LogEntry>>(&file_content) {
                Ok(log_entries) => {
                    for log in log_entries {
                        if !uniqe_users.contains(&log.user_id) {
                            uniqe_users.push(log.user_id);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return (err_count, Vec::new());
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return (err_count, Vec::new());
        }
    }

    // Now we have the uniqe users count and the uniqe users names
    (uniqe_users.capacity(), uniqe_users)
}

fn read_json_file(file_path: &Path) -> Result<String, std::io::Error> {
    File::open(file_path).map(|file| {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        // For error handling
        match buf_reader.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(_) => {}
        }
        contents
    })
}
