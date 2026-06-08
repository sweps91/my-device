use std::thread;
use sysinfo::{
    Components, Disks, MINIMUM_CPU_UPDATE_INTERVAL, Networks, ProcessesToUpdate, System,
};

/// Create string with device monitoring data.
///
/// # Examples
/// ```rust
/// my_device::report::create_report(&"2026-06-01".to_string(), &"11h-11m-11s".to_string());
/// ```
pub fn create_report(day: &String, time: &String) -> (String, String) {
    let mut sys: System = System::new_all();

    // Update all information
    sys.refresh_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    // Create report mut variable for final reporting
    let host_name: String = extract_string("host_name", || System::host_name());
    let mut report: String = format!("MY DEVICE: {}\nday: {}\ntime: {}\n", host_name, day, time);

    // SYSTEM
    report += &report_system();

    // CPU (with required refreshes for time delta comparison)
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    report += &report_cpu(&mut sys);

    // RAM
    report += &report_ram(&mut sys);

    // DISKS
    report += &report_disks();

    // NETWORK
    report += &report_network();

    // PROCESSES
    report += &format!(
        "\nPROCESSES:\nNumber of processes: {:?}\n",
        sys.processes().len()
    );

    // TOP RAM PROCESSES
    report += &report_top_ram_processes(&mut sys, 20);

    // TOP CPU PROCESSES
    report += &report_top_cpu_processes(&mut sys, 10);

    // COMPONENTS
    report += &report_components();

    (host_name, report)
}

/// Transfer number from bytes and return as string in gigabytes.
fn b_to_gb(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f32 / 1024.0 / 1024.0 / 1024.0) // or num / 1_073_741_824
}

/// Get cpu usage.
fn cpu_usage(sys: &mut System) -> String {
    let cpu_usage = format!(
        "cpu usage: {:.1}% (bellow per unit)",
        sys.global_cpu_usage()
    );

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
         name:              {}\n\
         os version:        {}\n\
         os long version:   {}\n\
         kernel version:    {}\n\
         uptime (hours):    {}\n",
        extract_string("name", || System::name()),
        extract_string("os_version", || System::os_version()),
        extract_string("os_long_version", || System::long_os_version()),
        extract_string("kernel_version", || System::kernel_version()),
        System::uptime() / 3_600,
    )
}

/// Get CPU monitoring data.
fn report_cpu(sys: &mut System) -> String {
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
fn report_ram(sys: &mut System) -> String {
    format!(
        "\nRAM:\n\
         total memory: {}\n\
         used memory:  {}\n\
         total swap:   {}\n\
         used swap:    {}\n",
        b_to_gb(sys.total_memory()),
        b_to_gb(sys.used_memory()),
        b_to_gb(sys.total_swap()),
        b_to_gb(sys.used_swap()),
    )
}

/// Get data about device disks.
fn report_disks() -> String {
    let mut report: String = format!("\nDISKS:\n");
    let disks: Disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        report += &format!(
            "{:?} - total: {}, free: {}, removable: {}, file system: {:?}, mounted on: {:?}\n",
            disk.name(),
            b_to_gb(disk.total_space()),
            b_to_gb(disk.available_space()),
            disk.is_removable(),
            disk.file_system(),
            disk.mount_point()
        );
    }
    report
}

/// Get data about connected network.
fn report_network() -> String {
    let mut report = format!("\nNETWORK:\n");
    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        report += &format!(
            "{}: downloading: {:.2} MB, uploading: {:.2} MB\n",
            name,
            data.received() / 1024 / 1024,
            data.transmitted() / 1024 / 1024
        );
    }
    report
}

/// Get top RAM consuming processes.
fn report_top_ram_processes(sys: &mut System, top_num: usize) -> String {
    let mut report = format!("\nTOP {} RAM PROCESSES:\n", top_num);
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by_key(|b| std::cmp::Reverse(b.memory()));
    for p in processes.iter().take(top_num) {
        report += &format!("{:?}: {} MB\n", p.name(), p.memory() / 1_048_576);
    }
    report
}

/// Get top CPU consuming processes.
fn report_top_cpu_processes(sys: &mut System, top_num: usize) -> String {
    let mut report = format!("\nTOP {} CPU PROCESSES:\n", top_num);
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for p in processes.iter().take(top_num) {
        report += &format!("{:?}: {:.2}%\n", p.name(), p.cpu_usage());
    }
    report
}

/// Get components temperature
fn report_components() -> String {
    let components = Components::new_with_refreshed_list();

    let mut report = format!("\nCOMPONENTS:\n(need permission: run as administrator)\n");
    for component in &components {
        report += &format!("{component:?}");
    }
    report
}

// TESTS:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_to_gb() {
        let result = b_to_gb(16_782_584_709);
        assert_eq!(result, "15.63 GB");
    }
}
