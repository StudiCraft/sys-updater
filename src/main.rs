use std::{env,process::Command};
use sysinfo::System;
use log::{self, error, info, warn};
use env_logger::{self, Env};
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    let os_info = check_os();

    match args.len() {
        2 => {
            if args[1] == "update" {
                match os_info {
                    "arch" => {
                        info!("Updating for Arch-based distributions...");
                        update_arch();
                    }
                    "debian" => {
                        info!("Updating for Debian-based distributions...");
                        update_debian();
                    }
                    "gentoo" => {
                        warn!("Gentoo-based distributions are not yet supported for updating. The logic is here and almost ready to function properly.");
                    }
                    "fedora" => {
                        warn!("Fedora-based distributions are not yet supported for updating. The logic is here and almost ready to function properly.");
                    }
                    "opensuse" => {
                        warn!("SUSE-based distributions are not yet supported for updating. The logic is here and almost ready to function properly.");
                    }
                    "Windows" => {
                        warn!("Windows is not yet supported for updating. The logic is here and almost ready to function properly.");
                    }
                    "macOS" => {
                        warn!("macOS is not yet supported for updating. The logic is here and almost ready to function properly.");
                    }
                    "Unknown/OS" => {
                        error!("The program couldn't detect the operating system used.");
                    }
                    "Unknown/Linux" => {
                        error!("The program detected you were using Linux or a UNIX like operating system. Though it is not supported for updating. Please ask for developers or add yourself this ID to a family list: {}", System::distribution_id());
                    }
                    "BSD" => {
                        warn!("BSD systems are currently not supported. It is not prioritized but can be implemented in the future. Copy {} and ask a developer to implement it.", os_info);
                    }
                    _ => {
                        error!("{} is not a recognized operating system. You can ask developers to add it or do it yourself by copying this ID: {}", os_info, os_info);
                    }
                }
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
fn update_arch() {
    match Command::new("sudo")
        .arg("pacman")
        .arg("-Syu")
        .arg("--noconfirm")
        .status() {
            Ok(status) if status.success() => info!("Your system has been updated successfully!"),
            Ok(status) => error!("Your system failed to update and exited with error code: {}", status),
            Err(e) => error!("Pacman failed to start: {}", e)
        }
}
fn update_debian() {
    match Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .arg("-y")
        .status() {
            Ok(status) if status.success() => info!("Your system has been updated sucessfully!"),
            Ok(status) => error!("Your system failed to update with error code: {}", status),
            Err(e) => error!("Apt-get failed to start: {}", e)
        }
}
fn check_os() -> &'static str{
    match System::name().as_deref() {
        Some("Windows") => {
            return "Windows"
        }
        Some("Darwin") | Some("macOS") => {
            return "macOS"
        }
        Some("FreeBSD") | Some("NetBSD") | Some("OpenBSD") | Some("MidnightBSD") => {
            return "BSD"
        }
        Some(_) => {
            return check_linux();
        }
        None => {
            return "Unknown/OS"
        }
    }
}
fn check_linux() -> &'static str{
    let id = System::distribution_id().to_lowercase();

    const ARCH_DISTROS: &[&str] = &["arch", "cachyos", "endeavouros", "manjaro", "garuda", "steamos", "artix", "arcolinux", "rebornos"];
    const DEBIAN_DISTROS: &[&str] = &["debian", "ubuntu", "pop", "kali", "linuxmint", "zorin", "elementary", "parrot", "pureos"];
    const GENTOO_DISTROS: &[&str] = &["gentoo", "calculate", "redcore"];
    const FEDORA_DISTROS: &[&str] = &["fedora", "nobara", "rhel", "rocky", "almalinux"];
    const SUSE_DISTROS: &[&str] = &["opensuse-tumbleweed", "opensuse-leap", "opensuse"];

    if ARCH_DISTROS.contains(&id.as_str()) {
        return "arch"
    } else if DEBIAN_DISTROS.contains(&id.as_str()) {
        return "debian"
    } else if GENTOO_DISTROS.contains(&id.as_str()) {
        return "gentoo"
    } else if FEDORA_DISTROS.contains(&id.as_str()) {
        return "fedora"
    } else if SUSE_DISTROS.contains(&id.as_str()) {
        return "opensuse"
    } else {
        return "Unknown/Linux"
    }
}