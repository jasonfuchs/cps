use std::ffi::{c_int, CString};

use crate::prelude::*;

use pigpiod_if2::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pi(c_int);

impl Pi {
    pub fn new<A, P>(addr_str: A, port_str: P) -> Result<Pi>
    where
        A: ToString,
        P: ToString,
    {
        fn _new(addr_str: CString, port_str: CString) -> Result<Pi> {
            let pi = unsafe { pigpio_start(addr_str.as_ptr(), port_str.as_ptr()) };

            if pi < 0 {
                Err(Error::Pi(pi))
            } else {
                Ok(Pi(pi))
            }
        }

        _new(
            CString::new(addr_str.to_string()).map_err(|e| Error::other(e))?,
            CString::new(port_str.to_string()).map_err(|e| Error::other(e))?,
        )
    }
}
