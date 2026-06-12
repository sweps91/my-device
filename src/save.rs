use std::fs;

/// Save provided string to txt file.
pub(crate) fn save_report(folder: String, filename: String, report: String) {
    fs::create_dir_all(&folder).expect("Failed to create dir");
    fs::write(format!("{}/{}.txt", folder, filename), report).expect("Failed to write file");
}

// TESTS:
// TODO assert results

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_report() {
        save_report(
            "my-device-report".to_string(),
            "test".to_string(),
            "testing report".to_string(),
        )
    }
}
