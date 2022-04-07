
use sysinfo::{System, SystemExt,ProcessorExt,ComponentExt};
use std::collections::HashMap;


pub fn get_os_infos(sys: &System) -> HashMap<&'static str, Option<String>> {
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
    cpu_info.insert("cpu_freq",sys.global_processor_info().frequency().to_string());
    cpu_info.insert("cpu_brand",sys.global_processor_info().brand().to_string());
    cpu_info.insert("ram_size",(sys.total_memory() / 1000).to_string());
    cpu_info.insert("ram_used",(sys.used_memory() / 1000).to_string());
    cpu_info.insert("swap_size",(sys.total_swap() / 1000).to_string());
    cpu_info.insert("swap_used",(sys.used_swap() / 1000).to_string());

    return cpu_info;
}

pub fn get_adv_cpu_infos(sys:&System) -> HashMap<&'static str, Vec<String>> {

    let mut cpu_advanced:HashMap<&'static str,Vec<String>> = HashMap::new();
    let mut cpu_freqs = Vec::new();
    let mut comp_temp = Vec::new();
    let mut cpu_temps = Vec::new();

    for processor in sys.processors() {
        cpu_freqs.push(processor.frequency().to_string());
        comp_temp.push(processor.cpu_usage().to_string());

    }
    for comp in sys.components() {
        cpu_temps.push(comp.temperature().to_string());
    }

    cpu_advanced.insert("core_freqs",cpu_freqs);
    cpu_advanced.insert("comp_temps",comp_temp);
    cpu_advanced.insert("cpu_temps",cpu_temps);

    return cpu_advanced;

}