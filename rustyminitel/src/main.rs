// #[path = "./rusty_system/basic_sys.rs"]
// mod basic_sys;

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

    s.pop_layer();
    s.add_layer(
        Dialog::text("Informations User")
            .title("RustyMinitel / Informations")
            .button("Return Menu", menu),
    );
}

fn network(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Network User :")
            .title("RustyMinitel / Network")
            .button("Return Menu", menu)
            .biblionetwork()
            .fixed_size((75, 25)),
        );
}

fn biblionetwork (s: &mut Cursive) {

    let mut cpt = 1;
    let mut sys = System::new_all();

    sys.refresh_all();
    let networks = basic_network::get_networks(&sys);

    Dialog::text("\n\n ============== NETWORK INFO ===============");
        for net in networks{
            println!("\nNetwork n°{} : ",cpt);
            for(key,value) in net.into_iter(){
                print!("{} : {} ",key,value);
            }
            cpt+=1;
        } cpt = 0;
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
