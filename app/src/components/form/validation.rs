pub fn validate_required(value: &str, field_name: &str) -> Option<String> {
    if value.trim().is_empty() {
        Some(format!("{} is required", field_name))
    } else {
        None
    }
}

pub fn validate_email(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let at_pos = trimmed.find('@');
    match at_pos {
        Some(pos) if pos > 0 => {
            let domain = &trimmed[pos + 1..];
            if domain.contains('.') && domain.len() > 2 && !domain.ends_with('.') {
                None
            } else {
                Some("Please enter a valid email address".to_string())
            }
        }
        _ => Some("Please enter a valid email address".to_string()),
    }
}

pub fn validate_min_length(value: &str, min: usize, field_name: &str) -> Option<String> {
    if value.len() < min {
        Some(format!("{} must be at least {} characters", field_name, min))
    } else {
        None
    }
}

pub fn validate_max_length(value: &str, max: usize, field_name: &str) -> Option<String> {
    if value.len() > max {
        Some(format!("{} must be at most {} characters", field_name, max))
    } else {
        None
    }
}

pub fn validate_first(checks: &[Option<String>]) -> Option<String> {
    checks.iter().find_map(|c| c.clone())
}
