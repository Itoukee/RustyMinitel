mod second;
#[path = "./rusty_system/basic_sys.rs"]
mod basic_sys;
use sysinfo::{Processor, System, SystemExt};
use crate::basic_sys::get_adv_cpu_infos;

fn main() {
    let mut sys = System::new_all();
    let cpu = basic_sys::get_basic_cpu_infos(&sys);
    let adv_cpu = get_adv_cpu_infos(&sys);
    let base = basic_sys::get_os_infos(&sys);
    let mut cpt = 1;
    sys.refresh_all();


    println!("\n============== CPU BASE INFO ===============");
    for (key, value) in cpu.into_iter() {
        println!("{} : {}", key, value);

    }
    println!("\n============== CPU FULL INFO ===============");
    for (key, value) in adv_cpu.into_iter() {
        println!("{} : ",key);
        if key == "core_temps" {
            for i in value {
                print!("Core {} : {}°C ",cpt,i);
            }
            cpt = 1;
        }else if key == "core_freqs"{
            for i in value {
                print!("Core {} : {}MHz ",cpt,i);
                cpt+=1;
            }
            cpt = 1;
        }else if key=="comp_temps"{
            for i in value {
                print!("Component : {}°C ",i);

            }
        }


    }
    println!("\n============== SYS BASE INFO ===============");

    for(key,value) in base.into_iter(){
        match &value {
            Some(v)=> println!("{} : {:?}", key, &value),
            None => println!("{} : {}", key, "Null"),

        }
    }
}
