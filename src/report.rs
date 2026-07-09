use log::{debug, trace};
use std::env;
use std::thread;
use std::time::Duration;
use sysinfo::{Components, Disks, Networks, ProcessesToUpdate, System};

const MINIMUM_CPU_AND_NETWORK_UPDATE_INTERVAL: Duration = Duration::from_secs(1);

const TOP_RAM_PROCESSES: u8 = 25; // number of listed ram consuming processes
const TOP_CPU_PROCESSES: u8 = 15; // number of listed cpu consuming processes

/// Create string with device monitoring data.
///
/// Function also uses 1 second sleep.
///
/// # Examples
/// ```rust
/// my_device::report::create_report("2026-06-01", "11h-11m-11s", "+2:00");
/// ```
pub fn create_report(day: &str, time: &str, timezone: &str) -> (String, String) {
    let mut sys: System = System::new_all();
    trace!("Initializing system metrics: {:?}", sys);

    // Update all information
    sys.refresh_all();
    trace!("sys.refresh_all done");

    // sys.refresh_processes(ProcessesToUpdate::All, true);
    // trace!("sys.refresh_processes done");

    // Get networks
    let mut networks = Networks::new_with_refreshed_list();
    debug!("networks: {:?}", networks);

    // sleep & refreshes required for cpu and network metrics
    debug!(
        "MINIMUM_CPU_AND_NETWORK_UPDATE_INTERVAL: {:?}",
        MINIMUM_CPU_AND_NETWORK_UPDATE_INTERVAL
    );
    thread::sleep(MINIMUM_CPU_AND_NETWORK_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    trace!("sys.refresh_cpu_all done");
    sys.refresh_processes(ProcessesToUpdate::All, true);
    trace!("sys.refresh_processes done");

    // Create report mut variable for final reporting
    let host_name: String = extract_string("host_name", System::host_name);
    debug!("host_name: {}", host_name);
    let mut report: String = format!(
        "MY DEVICE: {}\nday: {}\ntime: {}\ntimezone: {}\n",
        host_name, day, time, timezone
    );

    // SYSTEM
    report += &report_system();
    trace!("System section appended");

    // CPU
    report += &report_cpu(&sys);
    trace!("CPU section appended");

    // RAM
    report += &report_ram(&sys);
    trace!("RAM section appended");

    // DISKS
    report += &report_disks();
    trace!("Disks section appended");

    // NETWORK
    report += &report_network(&mut networks);
    trace!("Network section appended");

    // PROCESSES
    report += &format!(
        "\nPROCESSES:\nNumber of processes: {:?}\n",
        sys.processes().len()
    );

    // TOP RAM PROCESSES
    debug!("TOP_RAM_PROCESSES: {}", TOP_RAM_PROCESSES);
    report += &report_top_ram_processes(&sys, TOP_RAM_PROCESSES as usize);
    trace!("RAM Processes section appended");

    // TOP CPU PROCESSES
    debug!("TOP_CPU_PROCESSES: {}", TOP_CPU_PROCESSES);
    report += &report_top_cpu_processes(&sys, TOP_CPU_PROCESSES as usize);
    trace!("CPU Processes section appended");

    // COMPONENTS
    report += &report_components();
    trace!("Components section appended");

    debug!("report done for: {}", host_name);

    (host_name, report)
}

/// Transfer number from bytes and return as string in gigabytes.
fn b_to_gb(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
}

fn count_percent(full_number: u64, count_number: u64) -> String {
    if full_number == 0 {
        return "total is zero (cannot count percent)".to_string();
    }

    format!(
        "{:.0} %",
        count_number as f64 / (full_number as f64 / 100.0)
    )
}

/// Get cpu usage.
fn cpu_usage(sys: &System) -> String {
    let cpu_usage = format!("cpu usage: {:.1}% (below per unit)", sys.global_cpu_usage());

    let mut cpu_unit_usage: String = String::new();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        cpu_unit_usage += &format!("\ncpu {}: {:.1}%", i + 1, cpu.cpu_usage());
    }

    cpu_usage + &cpu_unit_usage
}

/// Extract string or return not_found.
fn extract_string<F>(looking_for: &str, f: F) -> String
where
    F: Fn() -> Option<String>,
{
    f().unwrap_or_else(|| format!("{}_not_found", looking_for))
}

/// Get data about system.
fn report_system() -> String {
    format!(
        "\nSYSTEM:\n\
         name:              {} (arch: {})\n\
         os version:        {}\n\
         os long version:   {}\n\
         kernel version:    {}\n\
         uptime (hours):    {}\n",
        extract_string("name", System::name),
        env::consts::ARCH,
        extract_string("os_version", System::os_version),
        extract_string("os_long_version", System::long_os_version),
        extract_string("kernel_version", System::kernel_version),
        System::uptime() / 3_600,
    )
}

/// Get CPU monitoring data.
fn report_cpu(sys: &System) -> String {
    let cpu_info = if let Some(cpu) = sys.cpus().first() {
        format!(
            "brand:          {}\n\
             vendor_id:      {}\n",
            cpu.brand(),
            cpu.vendor_id(),
        )
    } else {
        "brand:          not found\n\
         vendor_id:      not found\n"
            .to_string()
    };

    format!(
        "\nCPU:\n\
         {}number of CPUs: {}\n\
         {}\n",
        cpu_info,
        sys.cpus().len(),
        cpu_usage(sys),
    )
}

/// Get RAM monitoring data.
fn report_ram(sys: &System) -> String {
    let total_memory: u64 = sys.total_memory();
    let used_memory: u64 = sys.used_memory();

    format!(
        "\nRAM:\n\
         total memory: {}\n\
         used memory:  {} ({})\n\
         total swap:   {}\n\
         used swap:    {}\n",
        b_to_gb(total_memory),
        b_to_gb(used_memory),
        count_percent(total_memory, used_memory),
        b_to_gb(sys.total_swap()),
        b_to_gb(sys.used_swap()),
    )
}

/// Get data about device disks.
fn report_disks() -> String {
    let mut report: String = "\nDISKS:\n".to_string();
    let disks: Disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        let total_space: u64 = disk.total_space();
        let used_space: u64 = total_space.saturating_sub(disk.available_space());

        report += &format!(
            "{}: {:?} - total: {}, used: {} ({})\n\t  - removable: {}, file sys: {:?}, on: {:?}\n",
            disk.kind(),
            disk.name(),
            b_to_gb(total_space),
            b_to_gb(used_space),
            count_percent(total_space, used_space),
            disk.is_removable(),
            disk.file_system(),
            disk.mount_point(),
        );
    }
    report
}

/// Get data about connected network.
fn report_network(networks: &mut Networks) -> String {
    networks.refresh(true);

    let mut report = "\nNETWORK:\n".to_string();
    for (name, data) in networks.iter() {
        report += &format!(
            "{}: downloading: {:.3} KB, uploading: {:.3} KB\n",
            name,
            data.received() as f64 / 1024.0,
            data.transmitted() as f64 / 1024.0,
        );
    }
    report
}

/// Get top RAM consuming processes.
fn report_top_ram_processes(sys: &System, top_num: usize) -> String {
    let mut report = format!("\nTOP {} RAM PROCESSES:\n", top_num);
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by_key(|b| std::cmp::Reverse(b.memory()));
    for p in processes.iter().take(top_num) {
        report += &format!("{:?}: {} MB\n", p.name(), p.memory() / 1_048_576);
    }
    report
}

/// Get top CPU consuming processes.
fn report_top_cpu_processes(sys: &System, top_num: usize) -> String {
    let mut report = format!("\nTOP {} CPU PROCESSES:\n", top_num);
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for p in processes.iter().take(top_num) {
        report += &format!("{:?}: {:.1}%\n", p.name(), p.cpu_usage());
    }
    report
}

/// Get components temperature
fn report_components() -> String {
    let components = Components::new_with_refreshed_list();

    let mut report = "\nCOMPONENTS:\n(need permission: run as administrator)\n".to_string();
    for component in &components {
        report += &format!("{component:?}\n");
    }
    report
}

// TESTS:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_to_gb() {
        let result: String = b_to_gb(16_782_584_709);
        assert_eq!(result, "15.63 GB");
    }

    #[test]
    fn test_count_percent() {
        let result: String = count_percent(20, 10);
        assert_eq!(result, "50 %")
    }

    #[test]
    fn test_count_percent_zero() {
        let result: String = count_percent(0, 10);
        assert_eq!(result, "total is zero (cannot count percent)")
    }

    #[test]
    fn test_cpu_usage() {
        let mut sys: System = System::new_all();
        assert!(cpu_usage(&mut sys).contains("cpu usage"));
    }
}
