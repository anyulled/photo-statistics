use rayon::prelude::*;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use rusqlite::Connection;
use crate::config::Config;
use crate::exiftool::run_exiftool;
use crate::database::insert_metadata;

pub fn process_files_in_parallel(files: Vec<String>, config: &Config) -> Result<(), String> {
    if files.is_empty() {
        return Ok(());  
    }

    let conn = match Connection::open(&config.database_path) {
        Ok(conn) => Mutex::new(conn),
        Err(err) => {
            let error_msg = format!("Error opening database: {:?}", err);
            eprintln!("{}", error_msg);
            return Err(error_msg);
        }
    };

    files.par_chunks(50).for_each(|chunk| {
        let metadata = match run_exiftool(chunk) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error running exiftool: {:?}", err);
                return;
            }
        };

        let mut db_conn = match conn.lock() {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("Error acquiring database lock: {:?}", err);
                return;
            }
        };

        let transaction = match db_conn.transaction() {
            Ok(tx) => tx,
            Err(err) => {
                eprintln!("Error starting transaction: {:?}", err);
                return;
            }
        };

        for (file, data) in chunk.iter().zip(metadata.iter()) {
            let mod_time = std::fs::metadata(file)
                .and_then(|m| m.modified())
                .unwrap_or(UNIX_EPOCH);

            let mod_time_secs = match mod_time.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs_f64(),
                Err(err) => {
                    eprintln!("Error calculating duration: {:?}", err);
                    0.0
                }
            };

            if let Err(err) = insert_metadata(&transaction, file, mod_time_secs, data) {
                eprintln!("Error inserting metadata: {:?}", err);
            }
        }

        if let Err(err) = transaction.commit() {
            eprintln!("Error committing transaction: {:?}", err);
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_worker_parallel_processing() {
        let files = vec!["test1.jpg".to_string(), "test2.jpg".to_string()];
        let config = Config::new();
        let _result = process_files_in_parallel(files, &config);
    }

    #[test]
    fn test_empty_files_array() {
        let files: Vec<String> = vec![];
        let config = Config::new();
        let result = process_files_in_parallel(files, &config);
        assert!(result.is_ok(), "Processing empty files array should succeed");
    }

    #[test]
    fn test_nonexistent_files() {
        let files = vec!["nonexistent1.jpg".to_string(), "nonexistent2.jpg".to_string()];
        let config = Config::new();
        let result = process_files_in_parallel(files, &config);
        assert!(result.is_ok(), "Processing nonexistent files should not panic");
    }

    #[test]
    fn test_invalid_database_path() {
        let files = vec!["test1.jpg".to_string()];
        let mut config = Config::new();

        let temp_dir = tempdir().unwrap();
        let invalid_db_path = temp_dir.path().join("nonexistent_dir").join("db.sqlite");
        config.database_path = invalid_db_path;

        let result = process_files_in_parallel(files, &config);
        assert!(result.is_err(), "Processing with invalid database path should fail");
    }
}
