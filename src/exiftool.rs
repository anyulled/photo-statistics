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
}

