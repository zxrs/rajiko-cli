use crate::{
    prefecture::{AREA, Prefecture},
    statics::{APP_VERSION_MAP, ASMARTPHONE8_FULLKEY_B64, MODEL_LIST, VERSION_MAP},
    xml::{Prog, Programs, Radiko, Station, Station_, Stations, Urls},
};
use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, TimeDelta, Weekday};
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

fn generate_random_id() -> String {
    let hex = b"0123456789abcdef";
    (0..32)
        .map(|_| hex[rand::random_range(0..hex.len())] as char)
        .collect()
}

pub fn generate_random_info() -> Info {
    let version = VERSION_MAP[rand::random_range(0..VERSION_MAP.len())];
    let build = version.builds[rand::random_range(0..version.builds.len())];
    let model = MODEL_LIST[rand::random_range(0..MODEL_LIST.len())];
    let device = format!("{}.{}", version.sdk, model);
    let user_agent = format!(
        "Dalvik/2.1.0 (Linux; U; Android {}; {}/{})",
        version.id, model, build
    );
    let app_version = APP_VERSION_MAP[rand::random_range(0..APP_VERSION_MAP.len())];
    let user_id = generate_random_id();

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

pub fn login(pref: Prefecture) -> Result<(Client, Token)> {
    let info = generate_random_info();
    dbg!(&info);

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
    dbg!(&token, offset, len, &partial);

    let res = req
        .get(AUTH2_URL)
        .header("X-Radiko-App", info.app_version.1)
        .header("X-Radiko-App-Version", info.app_version.0)
        .header("X-Radiko-Device", info.device)
        .header("X-Radiko-User", info.user_id)
        .header("X-Radiko-AuthToken", token)
        .header("X-Radiko-PartialKey", partial)
        .header("X-Radiko-Location", pref.gen_gps())
        .send()?;
    dbg!(res);

    Ok((req, Token(token.to_str()?.into())))
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

fn playlist_url(station: &Station) -> Result<String> {
    let res = reqwest::blocking::get(format!(
        "https://radiko.jp/v3/station/stream/pc_html5/{}.xml",
        station.id
    ))?;
    let xml = res.text()?;
    let urls: Urls = serde_xml_rs::from_str(&xml)?;
    let playlist_url = urls
        .url
        .iter()
        .filter(|url| url.areafree.eq("0") && url.timefree.eq("1"))
        .map(|url| url.playlist_create_url.as_str())
        // .inspect(|v| println!("{v}"))
        .next()
        .unwrap_or("https://tf-f-rpaa-radiko.smartstream.ne.jp/tf/playlist.m3u8");
    Ok(playlist_url.into())
}

pub fn download(
    // req: &Client,
    pref: Prefecture,
    token: &Token,
    station: &Station,
    program: &Vec<Prog>,
) -> Result<()> {
    let playlist_url = playlist_url(station)?;
    dbg!(&playlist_url);

    let lsid = generate_random_id();

    const FIXED_SEEK: i64 = 300;

    let req = Client::builder().cookie_store(true).build()?;

    // let mut links = vec![];
    for p in program {
        let ft: DateTime<Local> = (&p.ft).try_into()?;
        let to: DateTime<Local> = (&p.to).try_into()?;

        let mut seek = ft.clone();

        while seek < to {
            let url = format!(
                "{}?lsid={}&station_id={}&l={FIXED_SEEK}&start_at={}&end_at={}&type=b&ft={2}&to={3}&seek={}",
                &playlist_url,
                &lsid,
                station.id,
                &ft.format("%Y%m%d%H%M%S"),
                &to.format("%Y%m%d%H%M%S"),
                &seek.format("%Y%m%d%H%M%S"),
            );
            dbg!(&url);
            let res = req
                .get(&url)
                .header("X-Radiko-AreaId", pref.id)
                .header("X-Radiko-AuthToken", &token.0)
                .send()?;
            dbg!(&res);

            let text = res.text()?;
            dbg!(text);

            seek = seek
                .checked_add_signed(TimeDelta::seconds(FIXED_SEEK))
                .context("date time is out out range")?;
        }
    }
    todo!()
}
