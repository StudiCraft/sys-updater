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
                        info!("Updating for Gentoo-based distributions...");
                        update_gentoo();
                    }
                    "fedora" => {
                        info!("Updating for Fedora-based distributions...");
                        update_fedora();
                    }
                    "opensuse" => {
                        info!("Updating for OpenSuse-based distributions...");
                        update_opensuse();
                    }
                    "windows" => {
                        info!("Updating for Windows system...");
                        update_windows();
                    }
                    "macos" => {
                        info!("Updating for MacOS system...");
                        warn!("Keep in mind that on MacOS systems, it will only update the core operating system or some Apple products like Safari, it will not update other applications.");
                        update_macos();
                    }
                    "unknown/os" => {
                        error!("The program couldn't detect the operating system used.");
                    }
                    "unknown/linux" => {
                        error!("The program detected you were using Linux or a UNIX like operating system. Though it is not supported for updating. Please ask for developers or add yourself this ID to a family list: {}", System::distribution_id());
                    }
                    "bsd" => {
                        warn!("BSD systems are currently not supported. It is not prioritized but can be implemented in the future. Copy {} and ask a developer to implement it.", os_info);
                    }
                    _ => {
                        error!("{} is not a recognized operating system. You can ask developers to add it or do it yourself by copying this ID: {}", os_info, os_info);
                    }
                }
            } else if args[1] == "help" {
                help();
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
        .status() {
            Ok(status) if status.success() => info!("Your system repositories have been updated successfully!"),
            Ok(status) => error!("Your system failed to update the repositories with error code: {}", status),
            Err(e) => error!("Apt-get failed to start: {}", e)
        }
    match Command::new("sudo")
        .arg("apt-get")
        .arg("dist-upgrade")
        .arg("-y")
        .status() {
            Ok(status) if status.success() => info!("Your system has been updated successfully!"),
            Ok(status) => error!("Your system failed to update with error code: {}", status),
            Err(e) => error!("Apt-get failed to start: {}", e)
        }
}
fn update_fedora() {
    match Command::new("sudo")
        .arg("dnf")
        .arg("upgrade")
        .arg("-y")
        .status() {
            Ok(status) if status.success() => info!("Your system has been updated successfully!"),
            Ok(status) => error!("Your system failed to update with error code: {}", status),
            Err(e) => error!("Dnf failed to start: {}", e)
        }
}
fn update_gentoo() {
    match Command::new("sudo")
        .arg("emaint")
        .arg("-a")
        .arg("sync")
        .status() {
            Ok(status) if status.success() => {
                info!("Your system repositories have been updated successfully!");
                update_gentoo_packages();
            }
            Ok(status) => error!("Your system failed to update the repositories with error code: {}", status),
            Err(e) => error!("Emaint failed to start: {}", e)
        }
}
fn update_gentoo_packages() {
    match Command::new("sudo")
        .arg("emerge")
        .arg("--update")
        .arg("--deep")
        .arg("--changed-use")
        .arg("--with-bdeps=y")
        .arg("--quiet-build=y")
        .arg("--keep-going=y")
        .arg("@world")
        .status() {
            Ok(status) if status.success() => info!("Your system has been updated successfully!"),
            Ok(status) => error!("Your system failed to update with error code: {}", status),
            Err(e) => error!("Emerge failed to start: {}", e)
        }
}
fn update_opensuse() {
    match System::distribution_id().to_lowercase().as_str() {
        "opensuse-tumbleweed" => {
            update_opensuse_tumbleweed();
        }
        "opensuse-leap" => {
            update_opensuse_leap();
        }
        _ => {
            return
        }
    }
}
fn update_opensuse_tumbleweed() {
    match Command::new("sudo")
        .arg("zypper")
        .arg("-n")
        .arg("dup")
        .status() {
        Ok(status) if status.success() => info!("Your system has been updated successfully!"),
        Ok(status) => error!("Your system failed to update with error code: {}", status),
        Err(e) => error!("Zypper failed to start: {}", e)
    }
}
fn update_opensuse_leap() {
    match Command::new("sudo")
        .arg("zypper")
        .arg("-n")
        .arg("up")
        .status() {
        Ok(status) if status.success() => info!("Your system has been updated successfully!"),
        Ok(status) => error!("Your system failed to update with error code: {}", status),
        Err(e) => error!("Zypper failed to start: {}", e)
    }
}
fn update_windows() {
    match Command::new("winget")
        .arg("upgrade")
        .arg("--all")
        .arg("--disable-interactivity")
        .status() {
        Ok(status) if status.success() => info!("Your system has been updated successfully!"),
        Ok(status) => error!("Your system failed to update with error code: {}", status),
        Err(e) => error!("Winget failed to start: {}", e)
    }
}
fn update_macos() {
    match Command::new("softwareupdate")
        .arg("--install")
        .arg("--all")
        .status() {
        Ok(status) if status.success() => info!("Your system has been updated successfully!"),
        Ok(status) => error!("Your system failed to update with error code: {}", status),
        Err(e) => error!("Softwareupdate failed to start: {}", e)
    }
}
fn check_os() -> &'static str{
    match System::name().as_deref() {
        Some("Windows") => {
            "windows"
        }
        Some("Darwin") | Some("macOS") => {
            "macos"
        }
        Some("FreeBSD") | Some("NetBSD") | Some("OpenBSD") | Some("MidnightBSD") => {
            "bsd"
        }
        Some(_) => {
            check_linux()
        }
        None => {
            "unknown/os"
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
        "arch"
    } else if DEBIAN_DISTROS.contains(&id.as_str()) {
        "debian"
    } else if GENTOO_DISTROS.contains(&id.as_str()) {
        "gentoo"
    } else if FEDORA_DISTROS.contains(&id.as_str()) {
        "fedora"
    } else if SUSE_DISTROS.contains(&id.as_str()) {
        "opensuse"
    } else {
        "unknown/linux"
    }
}