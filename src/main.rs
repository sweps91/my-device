use sysinfo::{Disks, Networks, System};

fn b_to_gb(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f32 / 1024.0 / 1024.0 / 1024.0)
}

fn main() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("MY DEVICE:\n");

    // Display system information:
    println!("SYSTEM:");
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    println!("from last start:         {} minutes", System::uptime() / 60);

    // RAM and swap information:
    println!("\nRAM:");
    println!("total memory: {}", b_to_gb(sys.total_memory()));
    println!("used memory : {}", b_to_gb(sys.used_memory()));
    println!("total swap  : {}", b_to_gb(sys.total_swap()));
    println!("used swap   : {}", b_to_gb(sys.used_swap()));

    // Number of CPUs:
    println!("\nCPU:");
    if let Some(cpu) = sys.cpus().first() {
        println!("brand:          {}", cpu.brand());
        println!("vendor_id:      {}", cpu.vendor_id());
    }
    println!("number of CPUs: {}", sys.cpus().len());

    println!("\nDISKS:");
    let disks: Disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "{:?} - total: {} GB, free: {} GB",
            disk.name(),
            disk.total_space() / 1_073_741_824,
            disk.available_space() / 1_073_741_824,
        );
    }

    println!("\nNETWORK:");
    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        println!(
            "{}: ↓{} KB  ↑{} KB",
            name,
            data.received() / 1024,
            data.transmitted() / 1024,
        );
    }

    println!("\nTOP 10 PROCESSES (RAM):");
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
    for p in processes.iter().take(10) {
        println!("{:?}: {} MB", p.name(), p.memory() / 1_048_576);
    }
}
