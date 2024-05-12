use anyhow::Result;
use cps::prelude::*;

fn main() -> Result<()> {
    let _sh_reg = ShiftRegister::<4>::builder()
        .addr("rpi07")
        .ds(17)
        .sh_cp(22)
        .st_cp(27)
        .build()?;

    Ok(())
}
