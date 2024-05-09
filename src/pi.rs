use std::ffi::{c_int, CString};

use crate::prelude::*;

use pigpiod_if2::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pi(c_int);

impl Pi {
    pub fn new<A, P>(addr_str: A, port_str: P) -> Result<Self>
    where
        A: ToString,
        P: ToString,
    {
        fn _new(addr_str: String, port_str: String) -> Result<Pi> {
            let addr_str = CString::new(addr_str).map_err(|nul_err| Error::other(nul_err))?;
            let port_str = CString::new(port_str).map_err(|nul_err| Error::other(nul_err))?;

            let pi = unsafe { pigpio_start(addr_str.as_ptr(), port_str.as_ptr()) };

            if pi.is_negative() {
                return Err(Error::Pi(pi));
            }

            Ok(Pi(pi))
        }

        _new(addr_str.to_string(), port_str.to_string())
    }
}
