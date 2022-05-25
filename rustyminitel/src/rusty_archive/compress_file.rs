use std::fs;
use std::path::Path;
use chrono::Local;
use std::process::Command;
use home::home_dir;

pub fn compress_file() {
    let nowaday = Local::now().to_rfc3339();
    let text = home_dir().unwrap();
    let locat = format!("{}/rustyminitel_infos",&text.display());

    let filename = format!{"{}/{}.rusty.tar.gz", &text.display(),nowaday};

    let mut tar_cmd = Command::new("tar");
    tar_cmd.arg("-czvf").arg(&filename).arg(&locat);
    tar_cmd.output().expect("failed to execute process");
    let path = Path::new(&locat);
    fs::remove_dir_all(path).expect("Error");
}