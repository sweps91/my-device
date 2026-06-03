mod report;
mod save;

const PRINT_REPORT: bool = true;
const SAVE_REPORT: bool = true;

/// main run
fn main() {
    let rep: String = report::create_report();

    if PRINT_REPORT {
        println!("{}", rep)
    }

    if SAVE_REPORT {
        save::save_report("my-device-report".to_string(), "report".to_string(), rep);
    }
}
