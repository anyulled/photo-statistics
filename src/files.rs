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
                let ext_lower = ext.to_lowercase();
                if allowed_extensions.contains(ext_lower.as_str()) {
                    files.push(entry.path().display().to_string());
                }
            }
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_scan_directory() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("image.jpg");
        fs::write(&file_path, "test").unwrap();

        let results = scan_directory(temp_dir.path().to_str().unwrap());
        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("image.jpg"));
    }

    #[test]
    fn test_scan_directory_no_images() {
        let temp_dir = tempdir().unwrap();
        let results = scan_directory(temp_dir.path().to_str().unwrap());
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_different_file_extensions() {
        let temp_dir = tempdir().unwrap();

        let jpg_path = temp_dir.path().join("image.jpg");
        let jpeg_path = temp_dir.path().join("image.jpeg");
        let cr2_path = temp_dir.path().join("image.cr2");
        let txt_path = temp_dir.path().join("image.txt"); // Not an image file

        fs::write(&jpg_path, "test").unwrap();
        fs::write(&jpeg_path, "test").unwrap();
        fs::write(&cr2_path, "test").unwrap();
        fs::write(&txt_path, "test").unwrap();

        let results = scan_directory(temp_dir.path().to_str().unwrap());
        assert_eq!(results.len(), 3); // Should find jpg, jpeg, and cr2, but not txt

        assert!(results.iter().any(|p| p.ends_with("image.jpg")));
        assert!(results.iter().any(|p| p.ends_with("image.jpeg")));
        assert!(results.iter().any(|p| p.ends_with("image.cr2")));
        assert!(!results.iter().any(|p| p.ends_with("image.txt")));
    }

    #[test]
    fn test_uppercase_extensions() {
        let temp_dir = tempdir().unwrap();

        let jpg_path = temp_dir.path().join("image.JPG");
        let jpeg_path = temp_dir.path().join("image.JPEG");

        fs::write(&jpg_path, "test").unwrap();
        fs::write(&jpeg_path, "test").unwrap();

        let results = scan_directory(temp_dir.path().to_str().unwrap());
        assert_eq!(results.len(), 2); // Should find both uppercase extensions

        assert!(results.iter().any(|p| p.ends_with("image.JPG")));
        assert!(results.iter().any(|p| p.ends_with("image.JPEG")));
    }

    #[test]
    fn test_subdirectories() {
        let temp_dir = tempdir().unwrap();

        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let main_jpg = temp_dir.path().join("main.jpg");
        let sub_jpg = sub_dir.join("sub.jpg");

        fs::write(&main_jpg, "test").unwrap();
        fs::write(&sub_jpg, "test").unwrap();

        let results = scan_directory(temp_dir.path().to_str().unwrap());
        assert_eq!(results.len(), 2); 

        assert!(results.iter().any(|p| p.ends_with("main.jpg")));
        assert!(results.iter().any(|p| p.ends_with("sub.jpg")));
    }

    #[test]
    fn test_nonexistent_directory() {
        let results = scan_directory("nonexistent_directory");
        assert_eq!(results.len(), 0); 
    }
}
