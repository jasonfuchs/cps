use std::{
    ffi::{c_char, c_int, c_uint, CString},
    path::Path,
};

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

    pub fn file_open<P>(&self, path: P, mode: c_uint) -> Result<File>
    where
        P: AsRef<Path>,
    {
        let path = CString::new(path.as_ref().to_str().ok_or(Error::other("invalid path"))?)?;

        let handle = unsafe { file_open(self.0, path.as_ptr().cast_mut(), mode) };

        if handle < 0 {
            return Err(Error::Pi(handle));
        }

        let handle = handle as c_uint;

        return Ok(File { handle, pi: &self });
    }

    pub fn file_read<const N: usize>(&self, file: &File, buf: &mut [c_char; N]) -> Result<()> {
        let count = N as c_uint;

        let result = unsafe { file_read(self.0, file.handle(), buf.as_mut_ptr(), count) };

        if result < 0 {
            return Err(Error::Pi(result));
        }

        Ok(())
    }

    pub fn file_close(&self, file: &File) -> Result<()> {
        let result = unsafe { file_close(self.0, file.handle()) };

        if result < 0 {
            return Err(Error::Pi(result));
        }

        Ok(())
    }
}

impl Drop for Pi {
    fn drop(&mut self) {
        unsafe { pigpio_stop(self.0) }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File<'a> {
    handle: c_uint,
    pi: &'a Pi,
}

impl<'a> File<'a> {
    pub fn handle(&self) -> c_uint {
        self.handle
    }

    pub fn read<const N: usize>(&self) -> Result<String> {
        let mut buf: [c_char; N] = [0; N];

        self.pi.file_read(self, &mut buf)?;

        let content = unsafe {
            CString::from_vec_unchecked(
                buf.into_iter()
                    .filter(|c| c != &0)
                    .map(|c| c as u8)
                    .collect(),
            )
        };

        Ok(content.to_str()?.to_string())
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.pi.file_close(self).unwrap()
    }
}
