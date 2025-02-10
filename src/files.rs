use walkdir::WalkDir;
use std::collections::HashSet;

const RAW_EXTENSIONS: [&str; 7] = ["cr2", "cr3", "nef", "arw", "raf", "dng", "rw2"];
const JPEG_EXTENSIONS: [&str; 2] = ["jpg", "jpeg"];

pub fn scan_directory(directory: &str) -> Vec<String> {
    let allowed_extensions: HashSet<&str> = RAW_EXTENSIONS.iter().chain(JPEG_EXTENSIONS.iter()).cloned().collect();
    let mut files = Vec::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {

        if entry.file_type().is_file() {

            if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                if allowed_extensions.contains(ext) {
                    files.push(entry.path().display().to_string());
                }
            }
        }
    }
    files
}
