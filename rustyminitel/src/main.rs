#[path = "./rusty_system/basic_sys.rs"]
mod basic_sys;

#[path = "./rusty_system/basic_process.rs"]
mod basic_process;

#[path = "./rusty_system/basic_network.rs"]
mod basic_network;


use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    let cpu = basic_sys::get_basic_cpu_infos(&sys);
    let adv_cpu = basic_sys::get_adv_cpu_infos(&sys);
    let base = basic_sys::get_os_infos(&sys);
    let processes = basic_process::get_all_process(&sys);
    let networks = basic_network::get_networks(&sys);

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
        cpt = 1;


    }
    println!("\n============== SYS BASE INFO ===============");

    for(key,value) in base.into_iter(){
        println!("{} : {:?}", key, &value);
    }


    println!("\n\n ============== PROCESSES INFO ===============");
    for proc in processes{
        println!("\nProcess n°{} : ",cpt);
        for(key,value) in proc.into_iter(){
            print!("{} : {} ",key,value);
        }
        cpt+=1;
    } cpt = 0;

    println!("\n\n ============== NETWORK INFO ===============");
    for net in networks{
        println!("\nNetwork n°{} : ",cpt);
        for(key,value) in net.into_iter(){
            print!("{} : {} ",key,value);
        }
        cpt+=1;
    } cpt = 0;


    println!("\n\n ============== TABLE ROUTES IP ===============");
    println!("{}",basic_network::get_ip_routes());




}
