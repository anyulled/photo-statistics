mod config;
mod database;
mod exiftool;
mod files;
mod statistics;
mod utils;
mod worker;

use config::Config;
use database::create_tables_if_needed;
use files::scan_directory;
use rusqlite::Connection;
use statistics::generate_statistics;
use std::env;
use std::time::Instant;
use worker::process_files_in_parallel;

fn main() {
    env_logger::init();

    let start_time = Instant::now();
    let directory = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    println!("📂 Processing directory: {}", directory);

    let config = Config::new();

    let conn = match Connection::open(&config.database_path) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening database: {:?}", err);
            eprintln!("Make sure the database path is valid and accessible.");
            return;
        }
    };

    if let Err(err) = create_tables_if_needed(&conn) {
        eprintln!("Error creating database tables: {:?}", err);
        eprintln!("The application may not function correctly without the required tables.");
    }

    println!("🔍 Scanning directory...");
    let files = scan_directory(&directory);

    println!("📷 Found {} files to process.", files.len());

    if files.is_empty() {
        println!("🚫 No valid images found.");
        return;
    }

    println!("🚀 Processing metadata...");
    if let Err(err) = process_files_in_parallel(files, &config) {
        eprintln!("Error processing files: {}", err);
        return;
    }

    println!("📊 Generating statistics...");
    generate_statistics(&conn);

    println!("✅ Completed in {:.2?}", start_time.elapsed());
}
