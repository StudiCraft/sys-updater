use std::{env,process::{Command, Stdio}};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help();
        }
        2 => {
            if args[1] == "update" {
                update();
            } else {
                help();
            }
        }
        _ => help()
    }
}
fn help() {
    const PKG: &str = env!("CARGO_PKG_NAME");
    const VER: &str = env!("CARGO_PKG_VERSION");
    println!("{} v{}", PKG, VER);
    println!("Please pass in an argument.");
    println!("Only Arch-based Linux distributions are currently supported.");
    println!("Available arguments are the following:");
    println!("sys-updater [ARGS]");
    println!("update : Updates the entire system.");
}
fn update() {
    Command::new("sudo")
        .arg("pacman")
        .arg("-Syu")
        .arg("--noconfirm")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("An error occured.");
}