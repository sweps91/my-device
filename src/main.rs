mod report;
mod save;

const PRINT_REPORT: bool = false;
const SAVE_REPORT: bool = false;

/// main run
fn main() {
    let rep: String = report::create_report();

    if PRINT_REPORT {
        println!("{}", rep)
    }

    if SAVE_REPORT {
        save::save_report();
    }
}
