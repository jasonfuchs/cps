use std::{
    ffi::{c_char, c_int, c_uint, CStr, CString},
    io,
    path::Path,
    ptr,
};

use crate::prelude::*;

use pigpiod_if2::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pi(c_int);

impl Default for Pi {
    fn default() -> Self {
        Pi(PI_NOT_INITIALISED)
    }
}

impl Pi {
    pub fn new() -> Result<Self> {
        Self::_pigpio_start(ptr::null(), ptr::null())
    }

    pub fn with_address(addr: &str) -> Result<Self> {
        Self::_with_address(&CString::new(addr)?)
    }

    fn _with_address(addr: &CStr) -> Result<Self> {
        Self::_pigpio_start(addr.as_ptr(), ptr::null())
    }

    pub fn with_address_and_port(addr: &str, port: u16) -> Result<Self> {
        Self::_with_address_and_port(&CString::new(addr)?, &CString::new(format!("{}", port))?)
    }

    fn _with_address_and_port(addr: &CStr, port: &CStr) -> Result<Self> {
        Self::_pigpio_start(addr.as_ptr(), port.as_ptr())
    }

    #[inline]
    fn _pigpio_start(addr: *const c_char, port: *const c_char) -> Result<Self> {
        let pi = unsafe { pigpio_start(addr, port) };

        if pi.is_negative() {
            Err(Error::Pi(pi))
        } else {
            Ok(Pi(pi))
        }
    }

    pub fn set_mode(&self, gpio: c_uint, mode: c_uint) -> Result<()> {
        let code = unsafe { set_mode(self.0, gpio, mode) };

        if code.is_negative() {
            Err(Error::Pi(code))
        } else {
            Ok(())
        }
    }

    pub fn gpio_write(&self, gpio: c_uint, level: c_uint) -> Result<()> {
        let code = unsafe { gpio_write(self.0, gpio, level) };

        if code.is_negative() {
            Err(Error::Pi(code))
        } else {
            Ok(())
        }
    }

    pub fn file_open<T>(&self, path: T, mode: c_uint) -> Result<File>
    where
        T: AsRef<Path>,
    {
        let path = CString::new(
            path.as_ref()
                .to_str()
                .ok_or(Error::other("failed to convert path"))?,
        )?;

        let handle = unsafe { file_open(self.0, path.as_ptr().cast_mut(), mode) };

        if handle.is_negative() {
            Err(Error::Pi(handle))
        } else {
            Ok(File(handle as c_uint, &self))
        }
    }

    pub fn file_read(&self, file: &File, buf: &mut [u8]) -> Result<usize> {
        let bytes =
            unsafe { file_read(self.0, file.0, buf.as_mut_ptr().cast(), buf.len() as c_uint) };

        if bytes.is_negative() {
            Err(Error::Pi(bytes))
        } else {
            Ok(bytes as usize)
        }
    }

    pub fn file_close(&self, file: &File) -> Result<()> {
        let result = unsafe { file_close(self.0, file.0) };

        if result.is_negative() {
            Err(Error::Pi(result))
        } else {
            Ok(())
        }
    }
}

impl Drop for Pi {
    fn drop(&mut self) {
        unsafe { pigpio_stop(self.0) }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File<'a>(c_uint, &'a Pi);

impl<'a> File<'a> {
    pub fn read<const N: usize>(&self) -> Result<String> {
        let mut buf = [0; N];

        let bytes = self.1.file_read(self, &mut buf)?;

        let v = buf.into_iter().take(bytes).collect();

        let content = unsafe { CString::from_vec_unchecked(v) };

        Ok(content.into_string()?)
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.1.file_close(self).unwrap()
    }
}

impl<'a> io::Read for File<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.1.file_read(self, buf).map_err(io::Error::other)
    }
}
