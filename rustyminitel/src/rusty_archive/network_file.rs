use std::fs::File;
use sysinfo::{ System, SystemExt };
use std::io::Write;
use home::home_dir;

#[path = "../rusty_system/basic_network.rs"]
mod basic_network;

pub fn network_file() { //-> std::io::Result<()> {
    let sys = System::new_all();
    let networks = basic_network::get_networks(&sys);

    let text = home_dir().unwrap();

    let mut locat = format!("{}/rustyminitel_infos/network_info.yml",&text.display());


    let mut file = File::create(locat).expect("create network_info.yml failed");
    

    // Setup all vec for networks infos
    let mut net_name = Vec::new();
    let mut net_data_received = Vec::new();
    let mut net_data_transmitted = Vec::new();
    let mut net_total_data_transmitted = Vec::new();

    let mut printable : String;

    for net in &networks {
        for (key, value) in net.into_iter() {
            let net_info = key.to_string();
            let net_value = value.to_string();

            if net_info == String::from("net_name") {
                net_name.push(net_value);
            } else if net_info == String::from("data_received") {
                net_data_received.push(net_value);
            } else if net_info == String::from("data_transmitted") {
                net_data_transmitted.push(net_value);
            } else if net_info == String::from("data_transmitted_total") {
                net_total_data_transmitted.push(net_value);
            }
        } 
    }

    file.write("\n---\n".as_bytes());
    for cpt in 0..net_name.len() {
        printable = format!("network number {}:\n", cpt);
        file.write(printable.as_bytes());
        printable = format!("\tname: {:?}\n", net_name[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\tdata received: {}\n", net_data_received[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\tdata transmitted: {}\n", net_data_transmitted[cpt]);
        file.write(printable.as_bytes());
        printable = format!("\ttotal data transmitted: {}\n", net_total_data_transmitted[cpt]);
        file.write(printable.as_bytes());
    }
}