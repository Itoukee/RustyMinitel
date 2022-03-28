
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt,ProcessorExt,Processor,Component,ComponentExt};
use std::collections::HashMap;


pub fn basic(mut sys: &System){
    if System::IS_SUPPORTED{

        println!("SUPER SYS");
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

    }else{
        println!("This os is not supported /!!");
    }
}

pub fn get_os_infos(mut sys: &System) -> HashMap<&'static str, Option<String>> {
    let mut os_info:HashMap<&'static str, Option<String>> = HashMap::new();
    os_info.insert("os_name",sys.name());
    os_info.insert("kernel_version",sys.kernel_version());
    os_info.insert("os_version",sys.os_version());
    os_info.insert("host_name",sys.host_name());

    return os_info;

}

pub fn get_basic_cpu_infos(sys: &System) -> HashMap<&'static str, String> {

    let mut cpu_info:HashMap<&'static str,String>= HashMap::new();

    cpu_info.insert("cpu_global_usage", sys.global_processor_info().cpu_usage().to_string());
    cpu_info.insert("cpu_cores",sys.processors().len().to_string());
    cpu_info.insert("cpu_name",sys.global_processor_info().name().to_string());
    cpu_info.insert("cpu_freq",sys.global_processor_info().frequency().to_string());



    return cpu_info;




}