use crate::{info, shell, success};
use colored::Colorize;
use regex::Regex;
use std::fs::{copy, remove_file};

pub fn install_m() {
    let mut s = shell!(
        "curl",
        vec!["https://api.github.com/repositories/500013933/releases/latest"]
    );

    let re = Regex::new("\"browser_download_url\":.\"(?P<url>.*darwin.zip)\"").unwrap();
    let vre = Regex::new(r"v(?P<version>\d.\d.\d)").unwrap();

    s = re
        .captures(&s)
        .unwrap()
        .name("url")
        .unwrap()
        .as_str()
        .to_string();

    let v = vre
        .captures(&s)
        .unwrap()
        .name("version")
        .unwrap()
        .as_str()
        .to_string();

    info![format!("Downloading oxido version {}", &v)];

    shell!("wget", vec![&s]);

    info!["Unpacking package"];

    shell!("unzip", vec!["oxido-darwin.zip"]);

    info!["Moving package"];

    copy(
        "oxido",
        format!("{}/.oxido/bin/oxido", std::env::var("HOME").unwrap()),
    )
    .unwrap();
    remove_file("oxido").unwrap();
    remove_file("oxido-darwin.zip").unwrap();

    success!["Oxido has been installed!"];
}
