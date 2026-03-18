mod login;
mod prefecture;

use anyhow::Result;
use login::login;

fn main() -> Result<()> {
    let token = login()?;
    Ok(())
}
