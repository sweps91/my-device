use log::{info, trace};
use std::fs;
use std::path::Path;

/// Save provided string to txt file.
pub(crate) fn save_report(folder: &str, filename: &str, report: &str) -> std::io::Result<()> {
    let target = Path::new(folder).join(format!("{filename}.txt"));

    fs::create_dir_all(folder)?;
    trace!("{} created or already existed", folder);

    fs::write(&target, report)?;
    info!("report saved to: {}", target.display());

    Ok(())
}

// TESTS:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_report() {
        assert!(save_report("my-device-report", "test", "testing report").is_ok());
    }
}
