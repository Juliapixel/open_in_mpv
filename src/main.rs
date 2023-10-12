use std::{io::Write, path::Path};
use env_logger::Env;
use log::{info, error};
use regex::Regex;

const ENTRY: &str = "\
[Desktop Entry]
Type=Application
NoDisplay=true
Name=MPV URL handler
Exec=\"{}\" %u
StartupNotify=false
Terminal=false
MimeType=x-scheme-handler/mpv;
";

static ENTRY_PATH: &str = "/usr/share/applications/mpv_url_handler.desktop";

fn add_desktop_entry() {
    let entry_str = ENTRY.replace("{}", format!("{}", std::env::current_exe().unwrap().display()).as_str());

    let desktop_entry = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(Path::new(ENTRY_PATH));

    match desktop_entry {
        Ok(mut entry) => {
            info!("desktop entry opened successfully");
            entry.write(entry_str.as_bytes()).expect("FAILED TO WRITE TO FILE");
            info!("desktop entry written to successfully");
        },
        Err(e) => {
            error!("failed to open or create desktop entry file!");
            panic!("{e:?}");
        },
    }
}

fn remove_desktop_entry() {
    match std::fs::remove_file(Path::new(ENTRY_PATH)) {
        Ok(_) => info!("successfully removed .desktop entry"),
        Err(e) => error!("failed to remove .desktop entry\n{e}"),
    }
}

fn update_desktop_database() {
    let updater = std::process::Command::new("sudo")
        .arg("update-desktop-database").spawn();

    updater.unwrap().wait().unwrap();
    info!("desktop entry database updated successfully");
}


fn main() {
    env_logger::init_from_env(Env::new().default_filter_or("INFO"));

    let args: Vec<String> = std::env::args().collect();

    match args[1].as_str() {
        "register" => {
            add_desktop_entry();
            update_desktop_database();
        },
        "remove" => {
            remove_desktop_entry();
            update_desktop_database();
        }
        cmd => {
            info!("attempting to open {cmd}");
            let rule = Regex::new(r"^mpv:\/{0,3}(.*)").unwrap();
            match rule.captures(cmd) {
                Some(url) => {
                    let target = url.get(1).unwrap().as_str();
                    info!("opening {target} on mpv");
                    std::process::Command::new("mpv")
                        .arg(target).spawn().unwrap().wait().unwrap();
                    info!("finished playing")
                },
                None => {
                    error!("{cmd} is not a valid url")
                }
            }
            return;
        }
    }
}
