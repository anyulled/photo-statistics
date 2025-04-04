use serde_json::Value;
use std::process::{Command, Output};

fn execute_exiftool(file_paths: &[String]) -> std::io::Result<Output> {
    Command::new("exiftool")
        .args(&[
            "-json",
            "-DateTimeOriginal",
            "-Model",
            "-LensModel",
            "-ISO",
            "-ExposureTime",
            "-FNumber",
            "-FocalLength",
            "-Flash",
            "-WhiteBalance",
            "-ImageWidth",
            "-ImageHeight",
            "-FocalLengthIn35mmFormat",
        ])
        .args(file_paths)
        .output()
}

pub fn run_exiftool(file_paths: &[String]) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    if file_paths.is_empty() {
        return Err("No files provided to ExifTool".into());
    }

    let output = execute_exiftool(file_paths)?;
    let json_str = String::from_utf8(output.stdout)?;

    if json_str.trim().is_empty() {
        return Err("ExifTool returned empty output".into());
    }
    let metadata: Vec<Value> = serde_json::from_str(&json_str)?;

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_exiftool_success() {
        let result = serde_json::from_str::<Vec<Value>>(r#"[{"SourceFile": "test.jpg", "ISO": "100"}]"#).unwrap();
        assert_eq!(result[0]["ISO"], "100");
    }

    #[test]
    fn test_run_exiftool_fail() {
        let empty_json = "";
        let result = serde_json::from_str::<Vec<Value>>(empty_json);
        assert!(result.is_err(), "Expected failure due to empty JSON.");
    }

    #[test]
    fn test_empty_file_paths() {
        let empty_files: Vec<String> = vec![];
        let result = run_exiftool(&empty_files);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No files provided to ExifTool");
    }

    #[test]
    fn test_malformed_json() {
        let malformed_json = r#"{"this is not valid JSON"#;
        let result = serde_json::from_str::<Vec<Value>>(malformed_json);
        assert!(result.is_err(), "Expected failure due to malformed JSON.");
    }

    #[test]
    fn test_utf8_conversion_error() {
        let invalid_utf8 = vec![0xFF, 0xFF, 0xFF]; // Invalid UTF-8 bytes
        let result = String::from_utf8(invalid_utf8);
        assert!(result.is_err(), "Expected failure due to invalid UTF-8.");
    }
}
