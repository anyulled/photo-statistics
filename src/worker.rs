use rayon::prelude::*;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use rusqlite::{Connection, Transaction};
use crate::config::Config;
use crate::exiftool::run_exiftool;
use crate::database::insert_metadata;
use serde_json::Value;

fn init_database_connection(db_path: &std::path::Path) -> Result<Mutex<Connection>, String> {
    match Connection::open(db_path) {
        Ok(conn) => Ok(Mutex::new(conn)),
        Err(err) => {
            let error_msg = format!("Error opening database: {:?}", err);
            eprintln!("{}", error_msg);
            Err(error_msg)
        }
    }
}

fn get_file_mod_time_secs(file_path: &str) -> f64 {
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

fn process_chunk(chunk: &[String], conn: &Mutex<Connection>) {
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

    process_files_in_transaction(&transaction, chunk, &metadata);

    if let Err(err) = transaction.commit() {
        eprintln!("Error committing transaction: {:?}", err);
    }
}

fn process_files_in_transaction(transaction: &Transaction, files: &[String], metadata: &[Value]) {
    for (file, data) in files.iter().zip(metadata.iter()) {
        let mod_time_secs = get_file_mod_time_secs(file);

        if let Err(err) = insert_metadata(transaction, file, mod_time_secs, data) {
            eprintln!("Error inserting metadata: {:?}", err);
        }
    }
}

pub fn process_files_in_parallel(files: Vec<String>, config: &Config) -> Result<(), String> {
    if files.is_empty() {
        return Ok(());  
    }

    let conn = init_database_connection(&config.database_path)?;

    files.par_chunks(50).for_each(|chunk| {
        process_chunk(chunk, &conn);
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
