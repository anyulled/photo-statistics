use std::process::Command;
use serde_json::Value;

pub fn run_exiftool(file_paths: &[String]) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let output = Command::new("exiftool")
        .args(&[
            "-json",
            "-DateTimeOriginal", "-Model", "-LensModel",
            "-ISO", "-ExposureTime", "-FNumber",
            "-FocalLength", "-Flash", "-WhiteBalance",
            "-ImageWidth", "-ImageHeight", "-FocalLengthIn35mmFormat"
        ])
        .args(file_paths)
        .output()?;

    let json_str = String::from_utf8(output.stdout)?;
    let metadata: Vec<Value> = serde_json::from_str(&json_str)?;

    Ok(metadata)
}
