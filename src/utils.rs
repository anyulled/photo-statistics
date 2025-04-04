pub fn normalize_focal_length(focal: Option<&str>) -> String {
    if let Some(focal) = focal {
        let focal_clean = focal
            .trim()
            .to_lowercase()
            .replace("mm", "")
            .replace(",", ".")
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
    if let Some(wb_str) = wb {
        let wb_lower = wb_str.trim().to_lowercase();
        match wb_lower.as_str() {
            "auto" | "auto (ambience priority)" => "auto".to_string(),
            "daylight" => "daylight".to_string(),
            "cloudy" => "cloudy".to_string(),
            "fluorescent" => "fluorescent".to_string(),
            "tungsten" => "tungsten".to_string(),
            "shade" => "shade".to_string(),
            _ => "manual".to_string(),
        }
    } else {
        "manual".to_string()
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
    fn test_focal_length_with_spaces() {
        assert_eq!(normalize_focal_length(Some("50 mm")), "50 mm");
        assert_eq!(normalize_focal_length(Some("  24  mm  ")), "24 mm");
        assert_eq!(normalize_focal_length(Some("70.0 mm")), "70 mm");
    }

    #[test]
    fn test_focal_length_with_different_decimal_separators() {
        assert_eq!(normalize_focal_length(Some("50,5mm")), "50.5 mm"); // European decimal separator
        assert_eq!(normalize_focal_length(Some("24,0 mm")), "24 mm");
        assert_eq!(normalize_focal_length(Some("100,5")), "100.5 mm");
    }

    #[test]
    fn test_normalize_white_balance() {
        assert_eq!(normalize_white_balance(Some("Auto")), "auto");
        assert_eq!(normalize_white_balance(Some("Daylight")), "daylight");
        assert_eq!(normalize_white_balance(Some("Custom")), "manual");
        assert_eq!(normalize_white_balance(Some("Unknown")), "manual");
        assert_eq!(normalize_white_balance(None), "manual"); // Null input
    }

    #[test]
    fn test_white_balance_with_different_capitalizations() {
        assert_eq!(normalize_white_balance(Some("AUTO")), "auto");
        assert_eq!(normalize_white_balance(Some("auto")), "auto");
        assert_eq!(normalize_white_balance(Some("DaYLiGhT")), "daylight");
        assert_eq!(normalize_white_balance(Some("TUNGSTEN")), "tungsten");
    }

    #[test]
    fn test_white_balance_with_extra_spaces() {
        assert_eq!(normalize_white_balance(Some(" Auto ")), "auto");
        assert_eq!(normalize_white_balance(Some("  daylight  ")), "daylight");
        assert_eq!(normalize_white_balance(Some("\tshade\n")), "shade");
    }

    #[test]
    fn test_white_balance_ambience_priority() {
        assert_eq!(normalize_white_balance(Some("Auto (Ambience Priority)")), "auto");
        assert_eq!(normalize_white_balance(Some("auto (ambience priority)")), "auto");
        assert_eq!(normalize_white_balance(Some(" AUTO (AMBIENCE PRIORITY) ")), "auto");
    }
}
