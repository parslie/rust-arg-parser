pub fn validate_short(short: Option<&str>) -> bool {
    match short {
        None => true,
        Some(val) => !val.starts_with("-"),
    }
}
pub fn validate_long(long: Option<&str>) -> bool {
    match long {
        None => true,
        Some(val) => !val.starts_with("--"),
    }
}
