// #![warn(missing_docs)]

pub mod report;
pub mod save;

const PRINT_REPORT: bool = true;
const SAVE_REPORT: bool = true;

/// Orchestrate report creation & its printing or/and saving if allowed.
///
/// # Examples
/// ```rust
/// my_device::run();
/// ```
pub fn run() {
    let rep: String = report::create_report();

    if PRINT_REPORT {
        println!("{}", rep)
    }

    if SAVE_REPORT {
        save::save_report("my-device-report".to_string(), "report".to_string(), rep);
    }
}
