mod database;
mod exiftool;
mod files;
mod statistics;
mod worker;
mod utils;

use database::create_tables_if_needed;
use files::scan_directory;
use statistics::generate_statistics;
use worker::process_files_in_parallel;
use std::env;
use std::time::Instant;

fn main() {
    env_logger::init();

    let start_time = Instant::now();
    let directory = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    println!("📂 Processing directory: {}", directory);

    create_tables_if_needed().expect("Failed to create database tables.");

    println!("🔍 Scanning directory...");
    let files = scan_directory(&directory);

    println!("📷 Found {} files to process.", files.len());

    if files.is_empty() {
        println!("🚫 No valid images found.");
        return;
    }

    println!("🚀 Processing metadata...");
    process_files_in_parallel(files);

    println!("📊 Generating statistics...");
    generate_statistics();

    println!("✅ Completed in {:.2?}", start_time.elapsed());
}

