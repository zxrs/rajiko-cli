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

#[derive(Debug, Deserialize)]
pub struct Programs {
    pub date: Date,
    // #[serde(rename = "prog")]
    pub prog: Vec<Prog>,
}

#[derive(Debug, Deserialize)]
pub struct Prog {
    #[serde(rename = "@ft")]
    pub ft: String,
    #[serde(rename = "@to")]
    pub to: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Date(String);
