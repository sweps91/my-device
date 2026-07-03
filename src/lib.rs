// #![warn(missing_docs)]

use env_logger;
use log::{debug, info, trace};
pub mod report;
mod save;
use chrono::Local;

const PRINT_REPORT: bool = false;
const SAVE_REPORT: bool = false;

/// Orchestrate report creation & its printing or/and saving if allowed.
///
/// # Examples
/// ```rust
/// my_device::run();
/// ```
pub fn run() {
    env_logger::init();
    info!("fn run starts ...");

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
