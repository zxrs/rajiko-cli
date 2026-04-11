mod prefecture;
mod statics;
mod util;
mod xml;

use anyhow::Result;
use util::{
    choose_date, choose_prefecture, choose_program, choose_realtime_program, choose_station,
    download_aac, login, part_links, real_time,
};

fn main() -> Result<()> {
    let pref = choose_prefecture()?;
    let token = login(pref)?;
    if real_time()? {
        let programs = choose_realtime_program(pref)?;
        return Ok(());
    }
    let station = choose_station(pref)?;
    let programs = choose_date(&station)?;
    let program = choose_program(&programs)?;
    let part_links = part_links(pref, &token, &station, &program)?;
    download_aac(&station, &program, part_links)?;
    Ok(())
}
