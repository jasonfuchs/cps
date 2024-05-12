use anyhow::Result;
use cps::{prelude::*, shift_register::ShiftRegister, wrapper::Wrap};

fn main() -> Result<()> {
    let _pi = Pi::with_port("rpi07", 2222)?;

    let _ = 0.wrap();

    let _sh_reg = ShiftRegister::<4>::builder()
        .addr("rpi07")
        .ds(17)
        .sh_cp(22)
        .st_cp(27)
        .build()?;

    Ok(())
}
