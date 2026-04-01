use anyhow::{Context, ensure};
use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stations {
    #[serde(rename = "station")]
    pub stations: Vec<Station>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub areafree: u8,
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
    #[serde(rename = "@id")]
    pub id: String,
    pub name: String,
    pub progs: Vec<Programs>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Programs {
    pub date: Date,
    // #[serde(rename = "prog")]
    pub prog: Vec<Prog>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prog {
    #[serde(rename = "@ft")]
    pub ft: Time,
    #[serde(rename = "@to")]
    pub to: Time,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Date(String);

impl TryFrom<&Date> for DateTime<Local> {
    type Error = anyhow::Error;
    fn try_from(value: &Date) -> Result<Self, Self::Error> {
        let time = &value.0;
        ensure!(time.len() == 8);

        let year = time.get(0..4).context("no year")?.parse()?;
        let month = time.get(4..6).context("no month")?.parse()?;
        let day = time.get(6..8).context("no day")?.parse()?;
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
pub struct Time(String);

impl TryFrom<&Time> for DateTime<Local> {
    type Error = anyhow::Error;
    fn try_from(value: &Time) -> Result<Self, Self::Error> {
        let time = &value.0;
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

#[derive(Debug, Deserialize)]
pub struct Urls {
    #[serde(rename = "url")]
    pub url: Vec<Url>,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    #[serde(rename = "@areafree")]
    pub areafree: String,
    #[serde(rename = "@max_delay")]
    pub max_delay: String,
    #[serde(rename = "@timefree")]
    pub timefree: String,
    pub playlist_create_url: String,
}
