mod login;
mod prefecture;
mod statics;
mod util;

use anyhow::Result;
use login::login;

fn main() -> Result<()> {
    let token = login()?;
    Ok(())
}
