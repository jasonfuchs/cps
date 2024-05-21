use std::{
    ffi::{c_char, c_int, c_uint, CString},
    io, mem,
    path::Path,
    ptr,
};

use crate::prelude::*;

use pigpiod_if2::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pi(c_int);

impl Pi {
    pub fn new(addr: &str) -> Result<Self> {
        let addr_str = CString::new(addr)?;

        let pi = unsafe { pigpio_start(addr_str.as_ptr(), ptr::null()) };

        if pi.is_negative() {
            Err(Error::Pi(pi))
        } else {
            Ok(Pi(pi))
        }
    }

    pub fn with_port(addr: &str, port: u16) -> Result<Self> {
        let addr_str = CString::new(addr)?;
        let port_str = CString::new(port.to_string())?;

        let pi = unsafe { pigpio_start(addr_str.as_ptr(), port_str.as_ptr()) };

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
            Ok(File {
                handle: handle as c_uint,
                pi: self,
            })
        }
    }

    pub fn file_read(&self, file: &File, buf: &mut [c_char]) -> Result<usize> {
        let read =
            unsafe { file_read(self.0, file.handle(), buf.as_mut_ptr(), buf.len() as c_uint) };

        if read.is_negative() {
            return Err(Error::Pi(read));
        } else {
            Ok(read as usize)
        }
    }

    pub fn file_close(&self, file: &File) -> Result<()> {
        let result = unsafe { file_close(self.0, file.handle()) };

        if result.is_negative() {
            return Err(Error::Pi(result));
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

impl<'a> io::Read for File<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.pi
            .file_read(self, unsafe { mem::transmute(buf) })
            .map_err(|err| io::Error::other(err))
    }
}
