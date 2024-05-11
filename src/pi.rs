use std::ffi::{c_char, c_int, c_uint, CStr, CString};

use crate::prelude::*;

use pigpiod_if2::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pi(c_int);

impl Pi {
    pub fn new<'a, A>(addr: A) -> Result<Self>
    where
        A: Into<&'a str>,
    {
        fn _new<'a>(addr: &'a str) -> Result<Pi> {
            Pi::from_ptr(CString::new(addr)?.as_ptr(), std::ptr::null())
        }

        _new(addr.into())
    }

    pub fn with_port<'a, A, P>(addr: A, port: P) -> Result<Self>
    where
        A: Into<&'a str>,
        P: Into<u16>,
    {
        fn _with_port<'a>(addr: &'a str, port: u16) -> Result<Pi> {
            Pi::from_ptr(
                CString::new(addr)?.as_ptr(),
                CString::new(port.to_string())?.as_ptr(),
            )
        }

        _with_port(addr.into(), port.into())
    }

    fn from_ptr(addr_str: *const c_char, port_str: *const c_char) -> Result<Self> {
        let pi = unsafe { pigpio_start(addr_str, port_str) };

        if pi < 0 {
            Err(Error::Pi(pi))
        } else {
            Ok(Pi(pi))
        }
    }

    pub fn set_mode(&self, gpio: c_uint, mode: c_uint) -> Result<()> {
        let code = unsafe { set_mode(self.0, gpio, mode) };

        if code < 0 {
            return Err(Error::Pi(code));
        }

        Ok(())
    }

    pub fn gpio_write(&self, gpio: c_uint, level: c_uint) -> Result<()> {
        let code = unsafe { gpio_write(self.0, gpio, level) };

        if code < 0 {
            return Err(Error::Pi(code));
        }

        Ok(())
    }
}
