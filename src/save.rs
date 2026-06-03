use std::fs;

pub fn save_report(folder: String, filename: String, report: String) {
    fs::create_dir_all(&folder).expect("Failed to create dir");
    fs::write(format!("{}/{}.txt", folder, filename), report).expect("Failed to write file");
}
