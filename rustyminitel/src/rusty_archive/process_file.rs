#[path = "../rusty_system/basic_process.rs"]
mod basic_process;

use std::fs::File;
use std::io::Write;
use home::home_dir;
use sysinfo::{System, SystemExt};

pub fn process_yml() {
    let text = home_dir().unwrap();

    let mut locat = format!("{}/rustyminitel_infos/process_info.yml",&text.display());
    let mut file = std::fs::File::create(locat).expect("create process.yml failed");
    let sys = System::new_all();
    let process_infos = basic_process::get_all_process(&sys);

    let mut proc_name = Vec::new();
    let mut proc_usage = Vec::new();
    let mut proc_pid = Vec::new();
    let mut proc_start = Vec::new();
    let mut proc_status = Vec::new();

    let mut printable : String;

    file.write("---\n".as_bytes());

    file.write("PROCESSES INFORMATIONS >\n".as_bytes());
        for proc in &process_infos {
            for (key, value) in proc.into_iter() {
                let proc_info = key.to_string();
                let proc_value = value.to_string();

                if proc_info == String::from("proc_name") {
                    proc_name.push(proc_value);
                } else if proc_info == String::from("proc_usage") {
                    proc_usage.push(proc_value);
                } else if proc_info == String::from("proc_pid") {
                    proc_pid.push(proc_value);
                } else if proc_info == String::from("proc_start") {
                    proc_start.push(proc_value);
                } else if proc_info == String::from("proc_status") {
                    proc_status.push(proc_value);
                }
            } 
}
    for cpt in 1..proc_name.len() {
        printable = format!("Number of processes : {}:\n", cpt);
        file.write(printable.as_bytes());
        printable = format!("\tName: {:?}\n", proc_name[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\tUsage: {}\n", proc_usage[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\tPID: {}\n", proc_pid[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\tStart: {}\n", proc_start[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\tStatus: {:?}\n\n", proc_status[cpt]);
        file.write(printable.as_bytes());
    }
}
