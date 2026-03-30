use crate::{
    prefecture::{AREA, Prefecture},
    statics::{APP_VERSION_MAP, ASMARTPHONE8_FULLKEY_B64, MODEL_LIST, VERSION_MAP},
    xml::{Prog, Programs, Radiko, Station, Station_, Stations},
};
use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, Weekday};
use reqwest::blocking::Client;
use std::io;

const AUTH1_URL: &str = "https://radiko.jp/v2/api/auth1";
const AUTH2_URL: &str = "https://radiko.jp/v2/api/auth2";

pub struct Token(String);

#[derive(Debug)]
pub struct Info {
    pub app_version: (&'static str, &'static str),
    pub user_id: String,
    pub user_agent: String,
    pub device: String,
}

pub fn generate_randam_info() -> Info {
    let version = VERSION_MAP[rand::random_range(0..VERSION_MAP.len())];
    let build = version.builds[rand::random_range(0..version.builds.len())];
    let model = MODEL_LIST[rand::random_range(0..MODEL_LIST.len())];
    let device = format!("{}.{}", version.sdk, model);
    let user_agent = format!(
        "Dalvik/2.1.0 (Linux; U; Android {}; {}/{})",
        version.id, model, build
    );
    let app_version = APP_VERSION_MAP[rand::random_range(0..APP_VERSION_MAP.len())];
    let hex = b"0123456789abcdef";
    let user_id = (0..32)
        .map(|_| hex[rand::random_range(0..hex.len())] as char)
        .collect();

    Info {
        app_version,
        user_id,
        user_agent,
        device,
    }
}

pub fn choose_prefecture() -> Result<Prefecture> {
    println!("Coose an area.");
    AREA.iter()
        .enumerate()
        .for_each(|(i, area)| println!("{:2}: {}", i + 1, area));
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let index = buf.trim().parse::<usize>()?;

    let area = AREA.get(index - 1).context("no area")?;

    println!("Choose a prefecture.");
    area.pref()
        .iter()
        .enumerate()
        .for_each(|(i, pref)| println!("{:2}: {}", i + 1, pref.name));
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let index = buf.trim().parse::<usize>()?;

    area.pref().get(index - 1).cloned().context("no prefecture")
}

pub fn login(pref: Prefecture) -> Result<Token> {
    let info = generate_randam_info();
    // dbg!(&info);

    let req = Client::builder().cookie_store(true).build()?;

    let res = req
        .get(AUTH1_URL)
        .header("X-Radiko-App", info.app_version.1)
        .header("X-Radiko-App-Version", info.app_version.0)
        .header("X-Radiko-Device", &info.device)
        .header("X-Radiko-User", &info.user_id)
        .send()?;

    let token = res
        .headers()
        .get("x-radiko-authtoken")
        .context("no auth token")?;
    let offset = res
        .headers()
        .get("x-radiko-keyoffset")
        .context("no key offset")?
        .to_str()?
        .parse::<usize>()?;
    let len = res
        .headers()
        .get("x-radiko-keylength")
        .context("no key length")?
        .to_str()?
        .parse::<usize>()?;
    let partial = ASMARTPHONE8_FULLKEY_B64
        .get(offset..offset + len)
        .context("no partial")?;

    _ = req
        .get(AUTH2_URL)
        .header("X-Radiko-App", info.app_version.1)
        .header("X-Radiko-App-Version", info.app_version.0)
        .header("X-Radiko-Device", info.device)
        .header("X-Radiko-User", info.user_id)
        .header("X-Radiko-AuthToken", token)
        .header("X-Radiko-PartialKey", partial)
        .header("X-Radiko-Location", pref.gen_gps())
        .send()?;

    Ok(Token(token.to_str()?.into()))
}

pub fn choose_station(pref: Prefecture) -> Result<Station> {
    let res = reqwest::blocking::get(format!("https://radiko.jp/v3/station/list/{}.xml", pref.id))?;
    let xml = res.text()?;
    let stations: Stations = serde_xml_rs::from_str(&xml)?;
    println!("Choose a station.");
    stations.stations.iter().enumerate().for_each(|(i, s)| {
        println!("{:2}: {}", i + 1, s.name);
    });
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let index = buf.trim().parse::<usize>()?;
    stations
        .stations
        .get(index - 1)
        .cloned()
        .context("no station")
}

pub fn choose_date(station: &Station) -> Result<Programs> {
    let res = reqwest::blocking::get(format!(
        "https://api.radiko.jp/program/v3/weekly/{}.xml",
        station.id
    ))?;
    let xml = res.text()?;
    let radiko: Radiko = serde_xml_rs::from_str(&xml)?;
    // dbg!(radiko);

    println!("Choose a date.");
    let programs: Vec<_> = radiko
        .stations
        .station
        .progs
        .into_iter()
        .filter_map(|p| -> Option<(DateTime<Local>, Programs)> {
            let date: DateTime<Local> = (&p.date).try_into().ok()?;
            if date < Local::now() {
                return Some((date, p));
            }
            None
        })
        .collect();

    programs.iter().enumerate().for_each(|(i, p)| {
        if p.0.weekday().eq(&Weekday::Sun) {
            println!(
                "\x1b[31m{:2}: {}\x1b[0m",
                i + 1,
                p.0.format("%Y-%m-%d (%A)")
            );
        } else if p.0.weekday().eq(&Weekday::Sat) {
            println!(
                "\x1b[34m{:2}: {}\x1b[0m",
                i + 1,
                p.0.format("%Y-%m-%d (%A)")
            );
        } else {
            println!("{:2}: {}", i + 1, p.0.format("%Y-%m-%d (%A)"));
        }
    });

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let index = buf.trim().parse::<usize>()?;

    programs
        .get(index - 1)
        .map(|p| p.1.clone())
        .context("no program")
}

pub fn choose_program(programs: &Programs) -> Result<Vec<Prog>> {
    println!("Choose a program. (eg: \"1 2 3\", \"10-12\")");
    programs
        .prog
        .iter()
        .enumerate()
        .try_for_each(|(i, p)| -> Result<()> {
            let ft: DateTime<Local> = (&p.ft).try_into()?;
            let to: DateTime<Local> = (&p.to).try_into()?;
            println!(
                "{:2}: {} 〜 {} {}",
                i + 1,
                ft.format("%H:%M"),
                to.format("%H:%M"),
                p.title
            );
            Ok(())
        })?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let buf = buf.trim();
    let index: Vec<(usize, usize)> = buf
        .split(" ")
        .filter_map(|v| {
            if v.contains("-") {
                let mut s = v.split("-");
                let start = s.next()?.parse::<usize>().ok()?;
                let end = s.next()?.parse::<usize>().ok()?;
                Some((start - 1, end - 1))
            } else {
                let start = v.parse::<usize>().ok()?;
                let end = v.parse::<usize>().ok()?;
                Some((start - 1, end - 1))
            }
        })
        .collect();

    let programs: Vec<_> = index
        .into_iter()
        .filter_map(|(start, end)| programs.prog.get(start..=end))
        .flatten()
        .cloned()
        .collect();
    Ok(programs)
}
