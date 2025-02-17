# 🦀 Exif Photo stats

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=anyulled_photo-statistics&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=anyulled_photo-statistics)
This is a rust crate that reads the EXIF data from the photos in a folder and prints some stats about it.

example:

```bash
📂 Processing directory: /some/photo/path
🔍 Scanning directory...
📷 Found 35 files to process.
🚀 Processing metadata...
📊 Generating statistics...
📊 ISO:
  N/A: 35

📊 Camera Models:
  N/A: 33
  NIKON Z 6_2: 2

📊 Lens Models:
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
