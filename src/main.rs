use anyhow::Result;
use cps::{prelude::*, wrapper::Wrap};

fn main() -> Result<()> {
    let _pi = Pi::new("rpi07")?;

    let _ = 0.wrap();

    Ok(())
}
