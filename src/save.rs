use log::{info, trace};
use std::fs;

/// Save provided string to txt file.
pub(crate) fn save_report(folder: String, filename: String, report: String) {
    let target: String = format!("{}/{}.txt", folder, filename);

    fs::create_dir_all(&folder).expect("Failed to create dir");
    trace!("{} created or already existed", folder);

    fs::write(&target, report).expect("Failed to write file");
    info!("report saved to: {}", target);
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
