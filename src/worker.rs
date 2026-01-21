//! Parallel file processing and database ingestion.
//!
//! This module handles the orchestration of reading files execution, extracted metadata,
//! and storing it in the SQLite database efficiently.

use rayon::prelude::*;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use rusqlite::Transaction;
use rusqlite::Connection; // Added this explicit import to match usage
use crate::config::Config;
use crate::exiftool::run_exiftool;
use crate::database::insert_metadata;
use serde_json::Value;
use crate::errors::{AppError, Result};

pub fn init_database_connection(db_path: &std::path::Path) -> Result<Mutex<Connection>> {
    let conn = Connection::open(db_path)?;
    Ok(Mutex::new(conn))
}

pub fn get_file_mod_time_secs(file_path: &str) -> f64 {
    let mod_time = std::fs::metadata(file_path)
        .and_then(|m| m.modified())
        .unwrap_or(UNIX_EPOCH);

    match mod_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(err) => {
            eprintln!("Error calculating duration: {:?}", err);
            0.0
        }
    }
}

pub fn process_chunk(chunk: &[String], conn: &Mutex<Connection>) -> Result<()> {
    let metadata = run_exiftool(chunk)?;
    
    let mut db_conn = conn.lock().map_err(|_| AppError::Processing("Database mutex poisoned".to_string()))?;

    let transaction = db_conn.transaction()?;

    process_files_in_transaction(&transaction, chunk, &metadata)?;

    transaction.commit()?;

    Ok(())
}

pub fn process_files_in_transaction(transaction: &Transaction, files: &[String], metadata: &[Value]) -> Result<()> {
    for (file, data) in files.iter().zip(metadata.iter()) {
        let mod_time_secs = get_file_mod_time_secs(file);

        insert_metadata(transaction, file, mod_time_secs, data)?;
    }

    Ok(())
}

pub fn process_files_in_parallel(files: Vec<String>, config: &Config) -> Result<()> {
    if files.is_empty() {
        return Ok(());  
    }

    let conn = init_database_connection(&config.database_path)?;

    // We collect errors to detect if any chunk failed, though ideally we might want to report all errors.
    // For now, we terminate on the first error found during iteration if we were to loop sequentially, 
    // but with par_chunks we need to decide how to aggregate.
    // The previous implementation suppressed errors.
    // We will use try_for_each to propagate the first error encountered.
    
    files.par_chunks(50).try_for_each(|chunk| {
        process_chunk(chunk, &conn)
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    fn setup_test_db() -> (tempfile::TempDir, std::path::PathBuf) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS metadata (
                source_file TEXT PRIMARY KEY,
                mod_time REAL,
                DateTimeOriginal TEXT,
                Model TEXT,
                LensModel TEXT,
                ISO TEXT,
                ExposureTime TEXT,
                FNumber TEXT,
                FocalLength TEXT,
                Flash TEXT,
                WhiteBalance TEXT,
                ImageWidth TEXT,
                ImageHeight TEXT,
                FocalLengthIn35mmFormat TEXT
            )",
            [],
        ).unwrap();

        (temp_dir, db_path)
    }

    #[test]
    fn test_worker_parallel_processing() {
        let _result = process_files_in_parallel(vec!["test1.jpg".to_string(), "test2.jpg".to_string()], &Config::new());
        assert!(_result.is_ok(), "Parallel processing should succeed");
    }

    #[test]
    fn test_empty_files_array() {
        let result = process_files_in_parallel(vec![], &Config::new());
        assert!(result.is_ok(), "Processing empty files array should succeed");
    }

    #[test]
    fn test_nonexistent_files() {
        let result = process_files_in_parallel(vec!["nonexistent1.jpg".to_string(), "nonexistent2.jpg".to_string()], &Config::new());
        assert!(result.is_ok(), "Processing nonexistent files should not panic");
    }

    #[test]
    fn test_invalid_database_path() {
        let mut config = Config::new();

        let invalid_db_path = tempdir()
            .unwrap()
            .path()
            .join("nonexistent_dir")
            .join("db.sqlite");
        config.database_path = invalid_db_path;

        let result = process_files_in_parallel(vec!["test1.jpg".to_string()], &config);
        assert!(result.is_err(), "Processing with invalid database path should fail");
    }

    #[test]
    fn test_init_database_connection_success() {
        let (_temp_dir, db_path) = setup_test_db();
        let result = init_database_connection(&db_path);
        assert!(result.is_ok(), "Database connection should succeed with valid path");
    }

    #[test]
    fn test_init_database_connection_failure() {
        let invalid_path = std::path::Path::new("/nonexistent/path/to/db.sqlite");
        let result = init_database_connection(invalid_path);
        assert!(result.is_err(), "Database connection should fail with invalid path");
    }

    #[test]
    fn test_get_file_mod_time_secs() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"test content").unwrap();
        }

        let mod_time = get_file_mod_time_secs(file_path.to_str().unwrap());
        assert!(mod_time > 0.0, "File modification time should be greater than 0");

        let nonexistent_file = "this_file_does_not_exist.jpg";
        let mod_time = get_file_mod_time_secs(nonexistent_file);
        assert_eq!(mod_time, 0.0, "Nonexistent file should return 0.0");
    }

    #[test]
    fn test_process_files_in_transaction() {
        let (_temp_dir, db_path) = setup_test_db();
        let mut connection = Connection::open(&db_path)
            .unwrap();
        let transaction = connection
            .transaction()
            .unwrap();

        let files = vec!["test1.jpg".to_string()];
        let metadata = vec![serde_json::json!({
            "DateTimeOriginal": "2023:01:01 12:00:00",
            "Model": "Test Camera"
        })];


        let result = process_files_in_transaction(&transaction, &files, &metadata);
        assert!(result.is_ok(), "Transaction should succeed");
    }
}
