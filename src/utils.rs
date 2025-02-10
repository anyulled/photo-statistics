pub fn normalize_focal_length(focal: Option<&str>) -> String {
    if let Some(focal) = focal {
        let focal_clean = focal.replace("mm", "").trim().to_string();
        if let Ok(value) = focal_clean.parse::<f64>() {
            return if value.fract() == 0.0 {
                format!("{} mm", value as i64)
            } else {
                format!("{:.1} mm", value)
            }
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
