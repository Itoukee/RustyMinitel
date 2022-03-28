use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

pub fn basic(){
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=> system:");
// RAM and swap information:
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("total swap  : {} KB", sys.total_swap());
    println!("used swap   : {} KB", sys.used_swap());

// Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

}