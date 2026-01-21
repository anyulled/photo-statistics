# 🦀 Exif Photo Statistics

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=anyulled_photo-statistics&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=anyulled_photo-statistics)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=anyulled_photo-statistics&metric=coverage)](https://sonarcloud.io/summary/new_code?id=anyulled_photo-statistics)

A high-performance Rust tool that scans directories for photos, extracts EXIF metadata (using ExifTool), and generates statistical insights.

## ✨ Features

- **Parallel Processing**: Uses `rayon` for multi-threaded file processing.
- **Resilient**: Robust error handling and SQLite storage for metadata caching.
- **Detailed Stats**: Analyzes ISO, Camera Models, Lens Models, Shutter Speed, and more.
- **Efficient**: Skips already processed files using modification time checks.

## 🚀 Requirements

- **Rust** (stable)
- **ExifTool**: Must be installed and available in your system path.
  - MacOS: `brew install exiftool`
  - Ubuntu: `sudo apt-get install libimage-exiftool-perl`

## 🛠️ Installation

```bash
cargo install --path .
```

## 📖 Usage

```bash
photo-statistics [OPTIONS] [DIRECTORY]
```

### Arguments

- `[DIRECTORY]`: The directory to scan for photos (default: current directory).

### Options

- `-d, --database <PATH>`: Path to the SQLite database file (default: `photo_stats_cache.db`).
- `-h, --help`: Print help.
- `-V, --version`: Print version.

### Example

```bash
$ photo-statistics ~/Pictures/Holiday2023
📂 Processing directory: /Users/alrs/Pictures/Holiday2023
🔍 Scanning directory...
📷 Found 1240 files to process.
🚀 Processing metadata...
📊 Generating statistics...

📊 Photos Per Year:
  2023: 1240

📊 Camera Models:
  NIKON Z 6_2: 1240

  NIKKOR Z 28-75mm f/2.8: 1
  NIKKOR Z 70-200mm f/2.8 VR S: 1
  N/A: 33

📊 Shutter Speed:
  1/200: 1
  1/100: 1
  N/A: 33

📊 Flash Usage:
  N/A: 33
  No Flash: 2

📊 Photos Per Year:
  N/A: 35

📊 Aperture:
  N/A: 35

📊 Focal Length:
  N/A: 33
  82 mm: 1
  57 mm: 1

📊 White Balance:
  manual: 34
  auto: 1

✅ Completed in 1.31s

Process finished with exit code 0
```

## 🧪 Development

### Running Tests

```bash
cargo test
```

### Formatting & Linting

```bash
cargo fmt
cargo clippy
```
