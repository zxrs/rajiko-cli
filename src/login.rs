use crate::{
    prefecture::Prefecture, statics::ASMARTPHONE8_FULLKEY_B64, util::generate_randam_info,
};
use anyhow::{Context, Result};
use reqwest::blocking::Client;

const AUTH1_URL: &str = "https://radiko.jp/v2/api/auth1";
const AUTH2_URL: &str = "https://radiko.jp/v2/api/auth2";

pub struct Token {}

pub fn login(pref: Prefecture) -> Result<Token> {
    let info = generate_randam_info();
    dbg!(&info);

    let req = Client::builder().cookie_store(true).build()?;

    let res = req
        .get(AUTH1_URL)
        .header("X-Radiko-App", info.app_version.1)
        .header("X-Radiko-App-Version", info.app_version.0)
        .header("X-Radiko-Device", info.device)
        .header("X-Radiko-User", info.user_id)
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

    todo!()
}
