use std::ffi::{c_int, c_uint, CString};

use crate::prelude::*;

use pigpiod_if2::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pi(c_int);

impl Pi {
    pub fn new<'a, A>(addr: A) -> Result<Self>
    where
        A: Into<&'a str>,
    {
        Self::_new(addr.into())
    }

    fn _new<'a>(addr: &'a str) -> Result<Self> {
        let addr_str = CString::new(addr)?;

        let pi = unsafe { pigpio_start(addr_str.as_ptr(), std::ptr::null()) };

        if pi < 0 {
            Err(Error::Pi(pi))
        } else {
            Ok(Pi(pi))
        }
    }

    pub fn with_port<'a, A>(addr: A, port: u16) -> Result<Self>
    where
        A: Into<&'a str>,
    {
        Self::_with_port(addr.into(), port)
    }

    fn _with_port<'a>(addr: &'a str, port: u16) -> Result<Self> {
        let addr_str = CString::new(addr)?;
        let port_str = CString::new(port.to_string())?;

        let pi = unsafe { pigpio_start(addr_str.as_ptr(), port_str.as_ptr()) };

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

impl Drop for Pi {
    fn drop(&mut self) {
        unsafe { pigpio_stop(dbg!(self).0) }
    }
}
