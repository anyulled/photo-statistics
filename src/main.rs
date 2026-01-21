mod config;
mod database;
mod errors;
mod exiftool;
mod files;
mod statistics;
mod utils;
mod worker;

use crate::config::{Cli, Config};
use crate::database::create_tables_if_needed;
use crate::files::scan_directory;
use crate::statistics::generate_statistics;
use crate::worker::process_files_in_parallel;
use anyhow::{Context, Result};
use clap::Parser;
use rusqlite::Connection;
use std::time::Instant;

fn main() -> Result<()> {
    env_logger::init();

    let start_time = Instant::now();
    let args = Cli::parse();
    
    println!("📂 Processing directory: {}", args.directory);

    let config = Config::from(args);

    let conn = Connection::open(&config.database_path)
        .context("Failed to open database")?;

    create_tables_if_needed(&conn)
        .context("Failed to create database tables")?;

    println!("🔍 Scanning directory...");
    let files = scan_directory(&config.directory);

    println!("📷 Found {} files to process.", files.len());

    if files.is_empty() {
        println!("🚫 No valid images found.");
        return Ok(());
    }

    println!("🚀 Processing metadata...");
    process_files_in_parallel(files, &config)
        .context("Failed to process files")?;

    println!("📊 Generating statistics...");
    generate_statistics(&conn);

    println!("✅ Completed in {:.2?}", start_time.elapsed());
    
    Ok(())
}
