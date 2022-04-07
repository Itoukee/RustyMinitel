#[path = "./rusty_system/basic_sys.rs"]
mod basic_sys;

#[path = "./rusty_system/basic_process.rs"]
mod basic_process;

#[path = "./rusty_system/basic_network.rs"]
mod basic_network;

use cursive::traits::*;
use cursive::view::Resizable;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

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
    let sys = System::new_all();
    let base = basic_sys::get_os_infos(&sys);
    let line = LinearLayout::horizontal();
    let buttons = LinearLayout::horizontal();
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
        Dialog::around(LinearLayout::horizontal().child(base_info))
            .title("RustyMinitel / Informations / System"),
    );
}

fn cpu_menu(s: &mut Cursive) {
    let sys = System::new_all();
    let cpu = basic_sys::get_basic_cpu_infos(&sys);
    let line = LinearLayout::horizontal();
    let buttons = LinearLayout::horizontal();
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
        Dialog::around(LinearLayout::horizontal().child(cpu_info))
            .title("RustyMinitel / Informations / CPU"),
    );
}

fn cpu_menu_more(s: &mut Cursive) {
    let sys = System::new_all();
    let adv_cpu = basic_sys::get_adv_cpu_infos(&sys);
    let mut line_1 = LinearLayout::horizontal();
    let mut line_2 = LinearLayout::horizontal();
    let mut line_3 = LinearLayout::horizontal();
    let mut buttons = LinearLayout::horizontal();
    let mut cpt;
    for (key, value) in adv_cpu.into_iter() {
        if key == "core_temps" {
            cpt = 1;
            for i in value {
                line_1.add_child(DummyView);
                let info_cpu = format!("Core {} : {}°C ", cpt, i);
                line_1.add_child(TextView::new(info_cpu));
            }
        } else if key == "core_freqs" {
            cpt = 1;
            for i in value {
                line_2.add_child(DummyView);
                let info_cpu = format!("Core {} : {}MHz ", cpt, i);
                line_2.add_child(TextView::new(info_cpu));
                cpt += 1;
            }
        } else if key == "comp_temps" {
            for i in value {
                line_3.add_child(DummyView);
                let info_cpu = format!("Component : {}°C ", i);
                line_3.add_child(TextView::new(info_cpu));
            }
        }
    }
    buttons.add_child(Button::new("System", information));
    buttons.add_child(DummyView);
    buttons.add_child(Button::new("Return Menu", menu));

    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(DummyView)
                .child(line_1)
                .child(DummyView)
                .child(line_2)
                .child(DummyView)
                .child(line_3)
                .child(DummyView)
                .child(buttons),
        )
        .title("RustyMinitel / Informations / CPU / More"),
    );
}

fn network(s: &mut Cursive) {
    let mut cpt = 1;
    let sys = System::new_all();
    let networks = basic_network::get_networks(&sys);
    let mut net_name_column = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Network Name  "))
        .child(DummyView);
    let mut data_received_column = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Data Received  "))
        .child(DummyView);
    let mut data_transmitted_column = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Data transmitted  "))
        .child(DummyView);
    let mut data_transmitted_total_column = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Total data transmitted  "))
        .child(DummyView);
    let mut return_button_layout = LinearLayout::vertical().child(DummyView);
    let mut netwo = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Network number  "))
        .child(DummyView);
    let mut ip_column = LinearLayout::horizontal();

    for net in networks {
        let network_info = format!("Network n°{} : ", cpt);
        netwo.add_child(TextView::new(network_info));
        netwo.add_child(DummyView);
        return_button_layout.add_child(DummyView);
        return_button_layout.add_child(DummyView);
        for (key, value) in net.into_iter() {
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
            } else if net_info == case_4 {
                data_transmitted_total_column.add_child(TextView::new(space_value));
                data_transmitted_total_column.add_child(DummyView);
            }
        }
        cpt += 1;
    }

    return_button_layout.add_child(Button::new("Return Menu", menu));
    let string_ip = format!("{}", basic_network::get_ip_routes());
    ip_column.add_child(TextView::new(string_ip));
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(netwo)
                        .child(net_name_column)
                        .child(data_received_column)
                        .child(data_transmitted_column)
                        .child(data_transmitted_total_column)
                        .child(return_button_layout),
                )
                .child(LinearLayout::horizontal().child(ip_column)),
        )
        .title("RustyMinitel / Network"),
    );
}

fn process(s: &mut Cursive) {
    let mut cpt = 1;
    let sys = System::new_all();
    let processes = basic_process::get_all_process(&sys);

    let buttons = LinearLayout::vertical()
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(TextView::new("-------"))
        .child(Button::new("Return to menu", menu))
        .child(Button::new("Kill a process", kill_process))
        .child(TextView::new("-------"));
    let mut process = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Process Number  "))
        .child(DummyView);
    let mut process_infos1 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Processes Name "))
        .child(DummyView);
    let mut process_infos2 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Processes Usage "))
        .child(DummyView);
    let mut process_infos3 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Processes PID "))
        .child(DummyView);
    let mut process_infos4 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Proceses Start "))
        .child(DummyView);
    let mut process_infos5 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Processes Runtime "))
        .child(DummyView);
    let mut process_infos6 = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new("Processes Status "))
        .child(DummyView);
    for proc in processes {
        let process_info = format!("Process n°{}", cpt);
        process.add_child(TextView::new(process_info));
        for (key, value) in proc.into_iter() {
            let key_string = key.to_string();
            let value_string = value.to_string();
            let space_value = format!("{} ", value_string);

            if key_string == "proc_name" {
                process_infos1.add_child(TextView::new(&space_value));
            }
            if key_string == "proc_usage" {
                process_infos2.add_child(TextView::new(&space_value));
            }
            if key_string == "proc_pid" {
                process_infos3.add_child(TextView::new(&space_value));
            }
            if key_string == "proc_start" {
                process_infos4.add_child(TextView::new(&space_value));
            }
            if key_string == "proc_runtime" {
                process_infos5.add_child(TextView::new(&space_value));
            }
            if key_string == "proc_status" {
                process_infos6.add_child(TextView::new(&space_value));
            }
        }
        cpt += 1;
        if cpt >= 46 {
            break;
        }
    }
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(process)
                .child(process_infos1)
                .child(process_infos2)
                .child(process_infos3)
                .child(process_infos4)
                .child(process_infos5)
                .child(process_infos6)
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

fn kill_process(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        let sys = System::new_all();
        kill_proc(&sys, name.parse().unwrap());
        s.pop_layer();
    }

    pub fn kill_proc(sys: &System, pid: i32) -> bool {
        return if let Some(process) = sys.process(Pid::from(pid)) {
            process.kill();
            true
        } else {
            false
        };
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("pid_kill")
                .fixed_width(10),
        )
        .title("Enter the PID")
        .button("Kill", |s| {
            let pid = s
                .call_on_name("pid_kill", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &pid);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}
