mod second;
#[path = "./rusty_system/basic_sys.rs"]
mod basic_sys;
use sysinfo::{Processor, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    println!("Hello, world!");
    let cpu = basic_sys::get_basic_cpu_infos(&sys);
    let base = basic_sys::get_os_infos(&sys);
    basic_sys::basic(&sys);
    for (key, value) in cpu.into_iter() {
        println!("{} : {}", key, value);

    }
}
