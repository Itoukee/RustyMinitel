#[path = "./rusty_system/basic_sys.rs"]
mod basic_sys;

#[path = "./rusty_system/basic_process.rs"]
mod basic_process;

#[path = "./rusty_system/basic_network.rs"]
mod basic_network;

// use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, TextView};
use cursive::Cursive;
use cursive::view::Resizable;
use sysinfo::{System, SystemExt};

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("Select :")
            .title("RustyMinitel")
            .button("Informations", information)
            .button("Network", network)
            .button("Process", process)
            .button("Quit", |_q| _q.quit()),
    );

    siv.run();
}

fn information(s: &mut Cursive) {
    let mut sys = System::new_all();
    let base = basic_sys::get_os_infos(&sys);
    let mut line = LinearLayout::horizontal();
    let mut buttons = LinearLayout::horizontal();
    let mut base_info = LinearLayout::vertical().child(line).child(buttons);

    for (key, value) in base.into_iter() {
        let info_string = format!("{} : {:?}", key.to_string(), value.unwrap());

        base_info.add_child(DummyView);
        base_info.add_child(TextView::new(info_string));
        base_info.add_child(DummyView);
    }

    base_info.add_child(Button::new("CPU", cpu_menu));
    base_info.add_child(Button::new("Return Menu", menu));

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(base_info),
        )
        .title("RustyMinitel / Informations / System"),
    );
}

fn cpu_menu(s: &mut Cursive) {
    let mut sys = System::new_all();
    let cpu = basic_sys::get_basic_cpu_infos(&sys);
    let mut line = LinearLayout::horizontal();
    let mut buttons = LinearLayout::horizontal();
    let mut cpu_info = LinearLayout::vertical().child(line).child(buttons);

    for (key, value) in cpu.into_iter() {
        let info_cpu = format!("{} : {}", key.to_string(), value.to_string());

        cpu_info.add_child(DummyView);
        cpu_info.add_child(TextView::new(info_cpu));
        cpu_info.add_child(DummyView);
    }

    cpu_info.add_child(Button::new("System", information));
    cpu_info.add_child(Button::new("CPU → More", cpu_menu_more));
    cpu_info.add_child(Button::new("Return Menu", menu));

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(cpu_info),
        )
        .title("RustyMinitel / Informations / CPU"),
    );
}

fn cpu_menu_more(s: &mut Cursive) {
    let mut sys = System::new_all();
    let adv_cpu = basic_sys::get_adv_cpu_infos(&sys);
    let mut line = LinearLayout::horizontal();
    let mut buttons = LinearLayout::horizontal();
    let mut adv_cpu_info = LinearLayout::vertical().child(line).child(buttons);
    let mut cpt = 1;

    for (key, value) in adv_cpu.into_iter() {
        if key == "core_temps" {
            cpt = 1;
            for i in value {
                adv_cpu_info.add_child(DummyView);
                let info_cpu = format!("Core {} : {}°C ", cpt, i);
                adv_cpu_info.add_child(TextView::new(info_cpu));
            }
            adv_cpu_info.add_child(DummyView);
        } else if key == "core_freqs"{
            cpt = 1;
            for i in value {
                adv_cpu_info.add_child(DummyView);
                let info_cpu = format!("Core {} : {}MHz ", cpt, i);
                adv_cpu_info.add_child(TextView::new(info_cpu));
                cpt+=1;
            }
            adv_cpu_info.add_child(DummyView);
        } else if key=="comp_temps"{
            cpt = 1;
            for i in value {
                adv_cpu_info.add_child(DummyView);
                let info_cpu = format!("Component : {}°C ", i);
                adv_cpu_info.add_child(TextView::new(info_cpu));
            }
            adv_cpu_info.add_child(DummyView);
        }
    }

    adv_cpu_info.add_child(Button::new("System", information));
    adv_cpu_info.add_child(Button::new("Return Menu", menu));

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(DummyView)
                .child(adv_cpu_info)
                .child(DummyView),
        )
        .title("RustyMinitel / Informations / CPU / More"),
    );
}

fn network(s: &mut Cursive) {
    let mut cpt = 1;
    let sys = System::new_all();
    let networks = basic_network::get_networks(&sys);
    let mut net_name_column = LinearLayout::vertical().child(DummyView).child(TextView::new("Network Name  ")).child(DummyView);
    let mut data_received_column = LinearLayout::vertical().child(DummyView).child(TextView::new("Data Received  ")).child(DummyView);
    let mut data_transmitted_column = LinearLayout::vertical().child(DummyView).child(TextView::new("Data transmitted  ")).child(DummyView);
    let mut data_transmitted_total_column = LinearLayout::vertical().child(DummyView).child(TextView::new("Total data transmitted  ")).child(DummyView);
    let mut return_button_layout = LinearLayout::vertical().child(DummyView);
    let mut netwo = LinearLayout::vertical().child(DummyView).child(TextView::new("Network number  ")).child(DummyView);
    let mut ip_column = LinearLayout::horizontal();

        for net in networks{
            let network_info = format!("Network n°{} : ", cpt);
            netwo.add_child(TextView::new(network_info));
            netwo.add_child(DummyView);
            return_button_layout.add_child(DummyView);
            return_button_layout.add_child(DummyView);
                for(key,value) in net.into_iter(){
                      let net_info = key.to_string();
                      let net_value = value.to_string();

                      let space_value = format!("{}  ", net_value);

                      let case_1 = String::from("net_name");
                      let case_2 = String::from("data_received");
                      let case_3 = String::from("data_transmitted");
                      let case_4 = String::from("data_transmitted_total");
                      if net_info == case_1 {
                          net_name_column.add_child(TextView::new(space_value));
                          net_name_column.add_child(DummyView);
                      } else if net_info == case_2 {
                          data_received_column.add_child(TextView::new(space_value));
                          data_received_column.add_child(DummyView);
                      } else if net_info == case_3 {
                        data_transmitted_column.add_child(TextView::new(space_value));
                        data_transmitted_column.add_child(DummyView);
                      } else if net_info == case_4{
                        data_transmitted_total_column.add_child(TextView::new(space_value));
                        data_transmitted_total_column.add_child(DummyView);
                      }  
                }
            cpt+=1;
        }

        return_button_layout.add_child(Button::new("Return Menu", menu));
        let string_ip = format!("{}", basic_network::get_ip_routes());
        ip_column.add_child(TextView::new(string_ip));
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(LinearLayout::horizontal()
                    .child(netwo)
                    .child(net_name_column)
                    .child(data_received_column)
                    .child(data_transmitted_column)
                    .child(data_transmitted_total_column)
                    .child(return_button_layout),
                )
                .child(LinearLayout::horizontal()
                        .child(ip_column),
                )

        )
            .title("RustyMinitel / Network")
        );
    }



fn process(s: &mut Cursive) {
    let mut cpt = 1;
    let sys = System::new_all();
    let processes = basic_process::get_all_process(&sys);

    let buttons = LinearLayout::vertical()
        .child(DummyView)
        .child(Button::new("Return to menu", menu));
    let process_infos1 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("test"));
    let process_infos2 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("test"));
    let mut process = LinearLayout::vertical().child(DummyView);
    for proc in processes {
        let process_info = format!("Process n°{}", cpt);
        process.add_child(Button::new(process_info, menu));
        // // for (key, value) in proc.into_iter() {
        //     print!("{} ", value);
        // }
        cpt += 1;
    }

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(process)
                .child(process_infos1)
                .child(process_infos2)
                .child(DummyView)
                .child(buttons),
        )
        .title("RustyMinitel / Process"),
    );
}

fn menu(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Select :")
            .title("RustyMinitel")
            .button("Informations", information)
            .button("Network", network)
            .button("Process", process)
            .button("Quit", |_q| _q.quit()),
    );
}

// fn main() {
//     let mut sys = System::new_all();
//     let cpu = basic_sys::get_basic_cpu_infos(&sys);
//     let adv_cpu = basic_sys::get_adv_cpu_infos(&sys);
//     let base = basic_sys::get_os_infos(&sys);
//     let processes = basic_process::get_all_process(&sys);
//     let networks = basic_network::get_networks(&sys);

//     let mut cpt = 1;

//     sys.refresh_all();

//     println!("\n============== CPU BASE INFO ===============");
//     for (key, value) in cpu.into_iter() {
//         println!("{} : {}", key, value);

//     }
//     println!("\n============== CPU FULL INFO ===============");
//     for (key, value) in adv_cpu.into_iter() {
//         println!("{} : ",key);
//         if key == "core_temps" {
//             for i in value {
//                 print!("Core {} : {}°C ",cpt,i);
//             }
//             cpt = 1;
//         }else if key == "core_freqs"{
//             for i in value {
//                 print!("Core {} : {}MHz ",cpt,i);
//                 cpt+=1;
//             }
//             cpt = 1;
//         }else if key=="comp_temps"{
//             for i in value {
//                 print!("Component : {}°C ",i);

//             }
//         }
//         cpt = 1;

//     }
//     println!("\n============== SYS BASE INFO ===============");

//     for(key,value) in base.into_iter(){
//         println!("{} : {:?}", key, &value);
//     }

//     println!("\n\n ============== PROCESSES INFO ===============");
//     for proc in processes{
//         println!("\nProcess n°{} : ",cpt);
//         for(key,value) in proc.into_iter(){
//             print!("{} : {} ",key,value);
//         }
//         cpt+=1;
//     } cpt = 0;

//     println!("\n\n ============== NETWORK INFO ===============");
//     for net in networks{
//         println!("\nNetwork n°{} : ",cpt);
//         for(key,value) in net.into_iter(){
//             print!("{} : {} ",key,value);
//         }
//         cpt+=1;
//     } cpt = 0;

//     println!("\n\n ============== TABLE ROUTES IP ===============");
//     println!("{}",basic_network::get_ip_routes());

// }
