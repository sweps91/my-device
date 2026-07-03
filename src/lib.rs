// #![warn(missing_docs)]

use env_logger;
use log::{debug, info, trace};
pub mod report;
mod save;
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
    env_logger::init();

    info!("fn run starts ...");

    let now = Local::now();
    debug!("now: {}", now);
    let day: String = format!("{}", now.format("%Y-%m-%d"));
    debug!("day: {}", day);
    let time: String = format!("{}", now.format("%Hh-%Mm-%Ss"));
    debug!("time: {}", time);
    let timezone: String = format!("UTC {}", now.format("%:z"));
    debug!("timezone: {}", timezone);

    let (host_name, rep) = report::create_report(&day, &time, &timezone);

    info!("PRINT_REPORT: {}", PRINT_REPORT);
    if PRINT_REPORT {
        println!("{}", rep);
        info!("report printed");
    }

    info!("SAVE_REPORT: {}", SAVE_REPORT);
    if SAVE_REPORT {
        save::save_report(
            "my-device-report".to_string(),
            format!("{}-{}-{}", host_name, day, time),
            rep,
        );
        info!("report saved");
    }

    info!("fn run done");
}
