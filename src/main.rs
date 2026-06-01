use sysinfo::{Components, Disks, Networks, System};

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
    println!(
        "from last start:         {} minutes",
        (System::uptime() % 3600) / 60
    );

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

    // // We display all disks' information:
    // println!("=> disks:");
    // let disks = Disks::new_with_refreshed_list();
    // for disk in &disks {
    //     println!("{disk:?}");
    // }

    // // Network interfaces name, total data received and total data transmitted:
    // let networks = Networks::new_with_refreshed_list();
    // println!("=> networks:");
    // for (interface_name, data) in &networks {
    //     println!(
    //         "{interface_name}: {} B (down) / {} B (up)",
    //         data.total_received(),
    //         data.total_transmitted(),
    //     );
    //     // If you want the amount of data received/transmitted since last call
    //     // to `Networks::refresh`, use `received`/`transmitted`.
    // }
}
