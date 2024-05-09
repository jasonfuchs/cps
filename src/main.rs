use anyhow::Result;
use cps::pi::Pi;

fn main() -> Result<()> {
    let _pi = Pi::new("rpi07", 8888)?;

    Ok(())
}
