use std::thread;
use sysinfo::{Disks, MINIMUM_CPU_UPDATE_INTERVAL, Networks, System};

/// create monitoring report
/// TODO
pub fn create_report() {
    let mut sys: System = System::new_all();

    // Update all information
    sys.refresh_all();

    println!("MY DEVICE:\n");

    // Display system information:
    println!("SYSTEM:");
    println!("System host name:        {:?}", System::host_name());
    println!("System name:             {:?}", System::name());
    println!("System OS version:       {:?}", System::os_version());
    println!("System OS long version:  {:?}", System::long_os_version());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System uptime (hours)    {:?}", System::uptime() / 3_600);

    // Number of CPUs:
    println!("\nCPU:");
    if let Some(cpu) = sys.cpus().first() {
        println!("brand:          {}", cpu.brand());
        println!("vendor_id:      {}", cpu.vendor_id());
    }
    println!("number of CPUs: {}", sys.cpus().len());
    println!("{}", cpu_usage(&mut sys));

    // RAM and swap information:
    println!("\nRAM:");
    println!("total memory: {}", b_to_gb(sys.total_memory()));
    println!("used memory : {}", b_to_gb(sys.used_memory()));
    println!("total swap  : {}", b_to_gb(sys.total_swap()));
    println!("used swap   : {}", b_to_gb(sys.used_swap()));

    println!("\nDISKS:");
    let disks: Disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "{:?} - total: {} GB, free: {} GB, removable: {}, file system: {:?}, mounted on: {:?}",
            disk.name(),
            b_to_gb(disk.total_space()),
            b_to_gb(disk.available_space()),
            disk.is_removable(),
            disk.file_system(),
            disk.mount_point()
        );
    }

    println!("\nNETWORK:");
    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        println!(
            "{}: downloading: {:.2} MB, uploading: {:.2} MB",
            name,
            data.received() / 1024 / 1024,
            data.transmitted() / 1024 / 1024,
        );
    }

    println!("\nPROCESSES:");
    println!("Number of processes: {:?}", sys.processes().len());

    println!("\nTOP 10 RAM PROCESSES:");
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by_key(|b| std::cmp::Reverse(b.memory()));
    for p in processes.iter().take(10) {
        println!("{:?}: {} MB", p.name(), p.memory() / 1_048_576);
    }

    println!("\nTOP 10 CPU PROCESSES:");
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for p in processes.iter().take(10) {
        println!("{:?}: {:.1}%", p.name(), p.cpu_usage());
    }
}

fn b_to_gb(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f32 / 1024.0 / 1024.0 / 1024.0) // or num / 1_073_741_824
}

fn cpu_usage(sys: &mut System) -> String {
    sys.refresh_cpu_all();

    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_all();

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
