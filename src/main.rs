mod prefecture;
mod statics;
mod util;
mod xml;

use anyhow::Result;
use util::{choose_date, choose_prefecture, choose_program, choose_station, download, login};

fn main() -> Result<()> {
    let pref = choose_prefecture()?;
    let (req, token) = login(pref)?;
    let station = choose_station(pref)?;
    let programs = choose_date(&station)?;
    let program = choose_program(&programs)?;
    dbg!(&station);
    dbg!(&program);
    download(pref, &token, &station, &program)?;

    Ok(())
}
