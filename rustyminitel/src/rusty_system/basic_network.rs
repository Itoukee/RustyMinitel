use sysinfo::{System, NetworksExt, Networks, NetworkData, SystemExt, NetworksIter, NetworkExt};
use std::collections::HashMap;
use std::process::Command;
use std::str::from_utf8;

pub fn get_networks(sys:&System) -> Vec<HashMap<&'static str, String>> {

    let networks = sys.networks();
    let mut net_advanced:Vec<HashMap<&'static str,String>> = Vec::new();
    let mut net_info:HashMap<&'static str,String>= HashMap::new();

    for (interface_name, data) in networks {
        net_info.insert("net_name", interface_name.to_string());
        net_info.insert("data_received",data.received().to_string());
        net_info.insert("data_transmitted",data.transmitted().to_string());
        net_info.insert("data_transmitted_total",data.total_transmitted().to_string());

        net_advanced.push(net_info);
        net_info = HashMap::new();
    }
    return net_advanced;
}

pub fn get_ip_routes() -> String {
    let mut command = Command::new("ip");
    command.args(["route","list"]);
    let output = command.output().unwrap();

    return (from_utf8(&output.stdout[..]).unwrap()).to_string();


}