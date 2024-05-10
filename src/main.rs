use std::ffi::CStr;

use anyhow::Result;
use cps::{pi::Pi, wrapper::W};

fn main() -> Result<()> {
    let err = Pi::new("rpi07", 8888).unwrap_err();

    let code = err.raw_pi_error().unwrap();

    let error = dbg!(unsafe { CStr::from_ptr(pigpiod_if2::pigpio_error(code)) });

    println!("{}", W(error).to_string());

    Ok(())
}
