use anyhow::Result;
use cps::prelude::*;

fn main() -> Result<()> {
    let _pi = Pi::new("rpi07")?;

    Ok(())
}
