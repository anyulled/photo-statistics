pub fn normalize_focal_length(focal: Option<&str>) -> String {
    if let Some(focal) = focal {
        let focal_clean = focal
            .trim()
            .to_lowercase()
            .replace("mm", "")
            .trim()
            .to_string();
        if let Ok(value) = focal_clean.parse::<f64>() {
            return if value.fract() == 0.0 {
                format!("{} mm", value as i64)
            } else {
                format!("{:.1} mm", value)
            };
        }
    }
    "N/A".to_string()
}

pub fn normalize_white_balance(wb: Option<&str>) -> String {
    let wb_lower = wb.unwrap_or("manual").to_lowercase();
    match wb_lower.as_str() {
        "auto" | "auto (ambience priority)" => "auto".to_string(),
        "daylight" => "daylight".to_string(),
        "cloudy" => "cloudy".to_string(),
        "fluorescent" => "fluorescent".to_string(),
        "tungsten" => "tungsten".to_string(),
        "shade" => "shade".to_string(),
        _ => "manual".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_focal_length() {
        assert_eq!(normalize_focal_length(Some("50mm")), "50 mm");
        assert_eq!(normalize_focal_length(Some("50.5mm")), "50.5 mm");
        assert_eq!(normalize_focal_length(Some(" 85 MM ")), "85 mm");
        assert_eq!(normalize_focal_length(Some("abc")), "N/A"); // Invalid input
        assert_eq!(normalize_focal_length(None), "N/A"); // Null input
    }

    #[test]
    fn test_normalize_white_balance() {
        assert_eq!(normalize_white_balance(Some("Auto")), "auto");
        assert_eq!(normalize_white_balance(Some("Daylight")), "daylight");
        assert_eq!(normalize_white_balance(Some("Custom")), "manual");
        assert_eq!(normalize_white_balance(Some("Unknown")), "manual");
        assert_eq!(normalize_white_balance(None), "manual"); // Null input
    }
}
