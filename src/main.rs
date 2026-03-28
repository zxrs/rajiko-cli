mod login;
mod prefecture;
mod statics;
mod util;

use anyhow::Result;
use login::login;
use prefecture::choose_prefecture;

fn main() -> Result<()> {
    let prefecture = choose_prefecture()?;
    let token = login(prefecture)?;
    Ok(())
}
