#[path = "../rusty_system/basic_sys.rs"]
mod basic_sys;

use std::fs::File;
use std::io::Write;
use home::home_dir;
use sysinfo::{System, SystemExt};

pub fn system_yml() {
    let text = home_dir().unwrap();

    let mut locat = format!("{}/rustyminitel_infos",&text.display());
    std::fs::create_dir(&locat).expect("Error couldn't create the dir");

    locat = format!("{}/system_info.yml", &locat);

    let mut file = std::fs::File::create(locat).expect("create system.yml failed");

    let sys = System::new_all();

    let os_infos = basic_sys::get_os_infos(&sys);
    let cpu_infos = basic_sys::get_basic_cpu_infos(&sys);
    let adv_cpu_infos = basic_sys::get_adv_cpu_infos(&sys);

    let mut cpt;

    file.write("---\n".as_bytes());

    file.write("</> OS INFORMATIONS >\n".as_bytes());
    for (key, value) in os_infos {
        if key == "os_name" || key == "host_name" {
            let data = format!("\t{}: {:?}\n", key.to_string(), value.as_ref().unwrap());
            file.write(data.as_bytes());
        }
        else {
            let data = format!("\t{}: {}\n", key.to_string(), value.as_ref().unwrap());
            file.write(data.as_bytes());
        }
    }

    file.write("\n</> CPU INFORMATIONS >\n".as_bytes());
    for (key, value) in cpu_infos {
        if key == "cpu_brand" {
            let data = format!("\t{}: {:?}\n", key.to_string(), value.to_string());
            file.write(data.as_bytes());
        }
        else {
            let data = format!("\t{}: {}\n", key.to_string(), value.to_string());
            file.write(data.as_bytes());
        }
    }

    file.write("\n< ADVANCED CPU INFORMATIONS >\n".as_bytes());
    for (key, value) in adv_cpu_infos {
        if key == "core_temps" {
            cpt = 1;
            for i in value {
                let data = format!("\tCore n°{}: {}°C\n", cpt, i);
                file.write(data.as_bytes());
            }
        } else if key == "core_freqs" {
            cpt = 1;
            for i in value {
                let data = format!("\tCore n°{}: {}MHz\n", cpt, i);
                file.write(data.as_bytes());
                cpt += 1;
            }
        } else if key == "comp_temps" {
            cpt = 1;
            for i in value {
                let data = format!("\tComponent n°{}: {}°C\n", cpt, i);
                file.write(data.as_bytes());
                cpt += 1;
            }
        }
    }
}
