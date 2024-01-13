use std::{path::Path, process::Command};

use regex::Regex;

use std::fs;

use sysinfo::{ProcessExt, System, SystemExt};

pub fn set_system_network() -> Result<(), Box<dyn std::error::Error>> {
    // let network_manager_dns_path = Path::new("/etc/NetworkManager/conf.d/dns.conf");
    // if !network_manager_dns_path.exists() {
    //     fs::File::create(network_manager_dns_path)?;
    // }
    // // https://github.com/YukiCoco/ToMoon/issues/7
    // // 判断 systemd-resolve 是否启动
    // if is_resolve_running() {
    //     // 对于新版本，使用 Issue 7 中的方案
    //     let default_config = "[Resolve]\nDNS=127.0.0.1:5354\nFallbackDNS=114.114.114.114\n";
    //     let resolved_dns_path = Path::new("/etc/systemd/resolved.conf");
    //     if !resolved_dns_path.exists() {
    //         fs::File::create(resolved_dns_path)?;
    //     }
    //     fs::write(resolved_dns_path, default_config)?;
    //     Command::new("systemctl")
    //         .arg("restart")
    //         .arg("systemd-resolved")
    //         .spawn()
    //         .unwrap()
    //         .wait()
    //         .unwrap();
    //     //将启用 systemd-resolved 写入 Network Manager
    //     let default_config = "[main]\ndns=systemd-resolved\n";
    //     fs::write(network_manager_dns_path, default_config)?;

    //     // 更新 NetworkManager
    //     Command::new("nmcli")
    //         .arg("general")
    //         .arg("reload")
    //         .spawn()
    //         .unwrap()
    //         .wait()
    //         .unwrap();
    // } else {
    //     // 旧版本的 SteamOS 没有 systemd-resolve
    //     // 修改 DNS 为可写
    //     Command::new("chattr")
    //         .arg("-i")
    //         .arg("/etc/resolv.conf")
    //         .spawn()
    //         .unwrap()
    //         .wait()
    //         .unwrap();
    //     // 修改系统 DNS 指向本地
    //     //fs::copy("/etc/resolv.conf", "./resolv.conf.bk")?;
    //     fs::write(
    //         "/etc/resolv.conf",
    //         "# Generated by ToMoon\nnameserver 127.0.0.1\n",
    //     )?;
    //     // 修改系统 DNS 为只读
    //     Command::new("chattr")
    //         .arg("+i")
    //         .arg("/etc/resolv.conf")
    //         .spawn()
    //         .unwrap()
    //         .wait()
    //         .unwrap();
    //     log::info!("Successfully set /etc/resolv.conf to read-only");
    //     //修改系统 DNS 默认设置
    //     let re = Regex::new(r"dns=(.+)").unwrap();
    //     let dns_config = fs::read_to_string(network_manager_dns_path)?;
    //     re.find(&dns_config);

    //     //关闭 systemd-resolved
    //     let mut sys = System::new_all();
    //     sys.refresh_all();
    //     for (_, process) in sys.processes() {
    //         if process.name() == "systemd-resolve" {
    //             log::info!("systemd-resolve is running");
    //             Command::new("systemctl")
    //                 .arg("disable")
    //                 .arg("--now")
    //                 .arg("systemd-resolved")
    //                 .spawn()
    //                 .unwrap()
    //                 .wait()
    //                 .unwrap();
    //             log::info!("Successfully disabled systemd-resolved");
    //         }
    //     }

    //     //下面的暂时无用
    //     match re.find(&dns_config) {
    //         Some(x) => {
    //             let match_dns_config = dns_config.get(x.start()..x.end()).unwrap();
    //             log::info!("Current dns config : {}", match_dns_config);
    //         }
    //         None => (),
    //     }

    //     //将默认 DNS 写入 Network Manager
    //     let default_config = "[main]\ndns=none\nsystemd-resolved=false\n";
    //     fs::write(network_manager_dns_path, default_config)?;

    //     // if !network_manager_dns_path.exists() {

    //     // }
    //     // if match_dns_config != "none" {
    //     //     let dns_config = dns_config.replace(match_dns_config, "none");
    //     //     fs::write("/etc/NetworkManager/conf.d/dns.conf", dns_config)?;
    //     // }

    //     // 更新 NetworkManager
    //     Command::new("nmcli")
    //         .arg("general")
    //         .arg("reload")
    //         .spawn()
    //         .unwrap()
    //         .wait()
    //         .unwrap();
    //     log::info!("Successfully refresh Network Manager");
    // }
    Ok(())
}

pub fn is_resolve_running() -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (_, process) in sys.processes() {
        if process.name() == "systemd-resolve" {
            return true;
        }
    }
    // First we update all information of our `System` struct.
    false
}

pub fn reset_system_network() -> Result<(), Box<dyn std::error::Error>> {
    //读入程序的 DNS
    let default_config = "[main]\ndns=auto";
    fs::write("/etc/NetworkManager/conf.d/dns.conf", default_config)?;
    // 修改 DNS 为可写
    Command::new("chattr")
        .arg("-i")
        .arg("/etc/resolv.conf")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    //fs::copy("./resolv.conf.bk", "/etc/resolv.conf")?;

    // 更新 NetworkManager
    Command::new("nmcli")
        .arg("general")
        .arg("reload")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    // match fs::copy("./resolv.conf.bk", "/etc/resolv.conf") {
    //     Ok(_) => (),
    //     Err(e) => {
    //         log::error!("reset_network() error: {}", e);
    //         return vec![];
    //     }
    // }
    log::info!("Successfully reset network");
    Ok(())
}

pub fn get_current_working_dir() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}

pub fn check_yaml(str: &String) -> bool {
    if let Ok(x) = serde_yaml::from_str::<serde_yaml::Value>(str) {
        if let Some(v) = x.as_mapping() {
            if v.contains_key("rules") {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn is_clash_running() -> bool {
    //关闭 systemd-resolved
    let mut sys = System::new_all();
    sys.refresh_all();
    for (_, process) in sys.processes() {
        if process.name() == "clash" {
            return true;
        }
    }
    return false;
}

// pub fn is_samrtdns_running() -> bool {
//     //关闭 systemd-resolved
//     let mut sys = System::new_all();
//     sys.refresh_all();
//     for (_, process) in sys.processes() {
//         if process.name() == "smartdns" {
//             return true;
//         }
//     }
//     return false;
// }

pub fn get_file_path(url: String) -> Option<String> {
    let r = Regex::new(r"^file://").unwrap();
    if let Some(x) = r.find(url.clone().as_str()) {
        let file_path = url[x.end()..url.len()].to_string();
        return Some(file_path);
    };
    return None;
}
