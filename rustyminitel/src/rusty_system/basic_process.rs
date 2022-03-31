use std::collections::HashMap;
use sysinfo::{/*Pid,*/ ProcessExt, System, SystemExt};

pub fn get_all_process(sys: &System) -> Vec<HashMap<&'static str, String>> {
    let mut proc_advanced: Vec<HashMap<&'static str, String>> = Vec::new();
    let mut proc_info: HashMap<&'static str, String> = HashMap::new();

    for (pid, proc) in sys.processes() {
        proc_info.insert("proc_name", proc.name().to_string());
        proc_info.insert("proc_usage", proc.cpu_usage().to_string());
        proc_info.insert("proc_pid", pid.to_string());
        proc_info.insert("proc_start", proc.start_time().to_string());
        proc_info.insert("proc_start", proc.run_time().to_string());
        proc_info.insert("proc_status", proc.status().to_string());

        proc_advanced.push(proc_info);

        proc_info = HashMap::new();
    }
    return proc_advanced;
}

// pub fn kill_proc(sys:&System, pid:i32) -> bool {
//     return if let Some(process) = sys.process(Pid::from(pid)) {
//         process.kill();
//         true
//     } else {
//         false
//     }
// }
