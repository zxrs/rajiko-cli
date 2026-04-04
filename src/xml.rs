use anyhow::{Context, ensure};
use chrono::prelude::*;
use serde::Deserialize;
use std::{fmt, ops::Deref};

#[derive(Debug, Deserialize)]
pub struct Stations {
    #[serde(rename = "station")]
    pub stations: Vec<Station>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    #[allow(unused)]
    pub areafree: u8,
    #[allow(unused)]
    pub timefree: u8,
}

#[derive(Debug, Deserialize)]
pub struct Radiko {
    // ttl: u32,
    // srvtime: u32,
    pub stations: Stations_,
}

#[derive(Debug, Deserialize)]
pub struct Stations_ {
    pub station: Station_,
}

#[derive(Debug, Deserialize)]
pub struct Station_ {
    #[allow(unused)]
    #[serde(rename = "@id")]
    pub id: String,
    #[allow(unused)]
    pub name: String,
    pub progs: Vec<Programs>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Programs {
    date: Date,
    prog: Vec<Prog>,
}

impl Programs {
    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn prog(&self) -> &[Prog] {
        self.prog.as_slice()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Date(String);

impl Date {
    pub fn to_datetime(&self) -> Result<DateTime<Local>, anyhow::Error> {
        self.try_into()
    }
}

impl Deref for Date {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&Date> for DateTime<Local> {
    type Error = anyhow::Error;
    fn try_from(date: &Date) -> Result<Self, Self::Error> {
        ensure!(date.len() == 8);

        let year = date.get(0..4).context("no year")?.parse()?;
        let month = date.get(4..6).context("no month")?.parse()?;
        let day = date.get(6..8).context("no day")?.parse()?;
        let time = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(year, month, day)
                    .context("invalid date")?
                    .and_time(Local::now().naive_local().time()),
            )
            .single()
            .context("no single time")?;

        Ok(time)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prog {
    #[serde(rename = "@ft")]
    ft: Time,
    #[serde(rename = "@to")]
    to: Time,
    title: String,
}

impl Prog {
    pub fn ft(&self) -> &Time {
        &self.ft
    }

    pub fn to(&self) -> &Time {
        &self.to
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Time(String);

impl Time {
    pub fn to_datetime(&self) -> Result<DateTime<Local>, anyhow::Error> {
        self.try_into()
    }
}

impl Deref for Time {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&Time> for DateTime<Local> {
    type Error = anyhow::Error;
    fn try_from(time: &Time) -> Result<Self, Self::Error> {
        ensure!(time.len() == 14);

        let year = time.get(0..4).context("no year")?.parse()?;
        let month = time.get(4..6).context("no month")?.parse()?;
        let day = time.get(6..8).context("no day")?.parse()?;
        let hour = time.get(8..10).context("no hour")?.parse()?;
        let min = time.get(10..12).context("no minutes")?.parse()?;
        let sec = time.get(12..14).context("no seconds")?.parse()?;

        let time = Local
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(year, month, day)
                    .context("invalid date")?
                    .and_hms_opt(hour, min, sec)
                    .context("invalid time")?,
            )
            .single()
            .context("no single time")?;
        Ok(time)
    }
}

impl fmt::Display for &Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct Urls {
    #[serde(rename = "url")]
    pub url: Vec<Url>,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    #[serde(rename = "@areafree")]
    pub areafree: String,
    #[allow(unused)]
    #[serde(rename = "@max_delay")]
    pub max_delay: String,
    #[serde(rename = "@timefree")]
    pub timefree: String,
    pub playlist_create_url: String,
}
