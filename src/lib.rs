// #![warn(missing_docs)]

pub mod report;
pub mod save;
use chrono::Local;

const PRINT_REPORT: bool = true;
const SAVE_REPORT: bool = true;

/// Orchestrate report creation & its printing or/and saving if allowed.
///
/// # Examples
/// ```rust
/// my_device::run();
/// ```
pub fn run() {
    let now = Local::now();
    let day: String = format!("{}", now.format("%Y-%m-%d"));
    let time: String = format!("{}", now.format("%Hh-%Mm-%Ss"));
    let timezone: String = format!("UTC {}", now.format("%:z"));

    let (host_name, rep) = report::create_report(&day, &time, &timezone);

    if PRINT_REPORT {
        println!("{}", rep)
    }

    if SAVE_REPORT {
        save::save_report(
            "my-device-report".to_string(),
            format!("{}-{}-{}", host_name, day, time),
            rep,
        );
    }
}
