use std::fs;

/// Save provided string to txt file.
///
/// # Examples
/// ```rust
/// my_device::save::save_report(
///     "my-device-report".to_string(),
///     "test".to_string(),
///     "testing report".to_string()
/// );
/// ```
pub fn save_report(folder: String, filename: String, report: String) {
    fs::create_dir_all(&folder).expect("Failed to create dir");
    fs::write(format!("{}/{}.txt", folder, filename), report).expect("Failed to write file");
}
