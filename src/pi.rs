use std::array;
use std::error;
use std::ffi;
use std::fmt;
use std::io;
use std::num;
use std::path;
use std::ptr;
use std::result;
use std::str;

#[derive(Debug)]
pub enum Error {
    Pi(&'static ffi::CStr),
    Other(Box<dyn error::Error + Send + Sync + 'static>),
}

pub type Result<T, E = Error> = result::Result<T, E>;

impl Error {
    pub fn new(errnum: ffi::c_int) -> Self {
        let err = unsafe { ffi::CStr::from_ptr(pigpiod_if2::pigpio_error(errnum)) };
        Self::Pi(err)
    }

    pub fn other<E>(error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync + 'static>>,
    {
        Self::Other(error.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pi(error) => error.to_bytes().escape_ascii().fmt(f),
            Self::Other(error) => error.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Other(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(error: ffi::NulError) -> Self {
        Self::other(error)
    }
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        Self::other(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::other(error)
    }
}

impl From<array::TryFromSliceError> for Error {
    fn from(error: array::TryFromSliceError) -> Self {
        Self::other(error)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Self::other(error)
    }
}

pub struct NoAddr;
pub struct NoPort;
#[derive(Debug)]
pub struct Uninit<A, P> {
    addr: A,
    port: P,
}
#[derive(Debug)]
pub struct Init(ffi::c_int);
#[derive(Debug)]
pub struct Pi<T>(T);

impl Pi<Uninit<NoAddr, NoPort>> {
    pub fn new() -> Self {
        Pi(Uninit {
            addr: NoAddr,
            port: NoPort,
        })
    }
}

impl Default for Pi<Uninit<NoAddr, NoPort>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, P> Pi<Uninit<A, P>> {
    pub fn addr(self, addr: &str) -> Pi<Uninit<&str, P>> {
        let Uninit { port, .. } = self.0;
        Pi(Uninit { addr, port })
    }

    pub fn port(self, port: &str) -> Pi<Uninit<A, &str>> {
        let Uninit { addr, .. } = self.0;
        Pi(Uninit { addr, port })
    }
}

impl Pi<Uninit<NoAddr, NoPort>> {
    pub fn connect(self) -> Result<Pi<Init>> {
        let pi = unsafe { pigpiod_if2::pigpio_start(ptr::null(), ptr::null()) };

        if pi.is_negative() {
            return Err(Error::new(pi));
        }

        Ok(Pi(Init(pi)))
    }
}

impl Pi<Uninit<&str, NoPort>> {
    pub fn connect(self) -> Result<Pi<Init>> {
        let Uninit { addr, .. } = self.0;
        let addr_str = ffi::CString::new(addr)?;

        let pi = unsafe { pigpiod_if2::pigpio_start(addr_str.as_ptr(), ptr::null()) };

        if pi.is_negative() {
            return Err(Error::new(pi));
        }

        Ok(Pi(Init(pi)))
    }
}

impl Pi<Uninit<NoAddr, &str>> {
    pub fn connect(self) -> Result<Pi<Init>> {
        let Uninit { port, .. } = self.0;
        let port_str = ffi::CString::new(port)?;

        let pi = unsafe { pigpiod_if2::pigpio_start(ptr::null(), port_str.as_ptr()) };

        if pi.is_negative() {
            return Err(Error::new(pi));
        }

        Ok(Pi(Init(pi)))
    }
}

impl Pi<Uninit<&str, &str>> {
    pub fn connect(self) -> Result<Pi<Init>> {
        let Uninit { addr, port } = self.0;
        let addr_str = ffi::CString::new(addr)?;
        let port_str = ffi::CString::new(port)?;

        let pi = unsafe { pigpiod_if2::pigpio_start(addr_str.as_ptr(), port_str.as_ptr()) };

        if pi.is_negative() {
            return Err(Error::new(pi));
        }

        Ok(Pi(Init(pi)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Gpio(ffi::c_uint);

impl Gpio {
    pub fn new(gpio: ffi::c_uint) -> Option<Self> {
        // documentation says only 0-53 allowed
        if gpio > 53 {
            return None;
        }

        Some(Gpio(gpio))
    }
}

// needed for command line argument parsing
impl str::FromStr for Gpio {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let raw = s.parse::<ffi::c_uint>()?;
        let gpio = Gpio::new(raw).ok_or(Error::new(pigpiod_if2::PI_BAD_GPIO))?;
        Ok(gpio)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GpioMode {
    Input = pigpiod_if2::PI_INPUT as isize,
    Output = pigpiod_if2::PI_OUTPUT as isize,
}

#[derive(Debug, Clone, Copy)]
pub enum GpioLevel {
    Low = pigpiod_if2::PI_LOW as isize,
    High = pigpiod_if2::PI_HIGH as isize,
}

impl Pi<Init> {
    pub fn try_with_addr(addr: &str) -> Result<Self> {
        Pi::new().addr(addr).connect()
    }

    pub fn try_with_addr_and_port(addr: &str, port: &str) -> Result<Self> {
        Pi::new().addr(addr).port(port).connect()
    }

    pub fn set_mode(&self, gpio: Gpio, mode: GpioMode) -> Result<()> {
        let err = unsafe { pigpiod_if2::set_mode(self.0 .0, gpio.0, mode as ffi::c_uint) };

        if err.is_negative() {
            return Err(Error::new(err));
        }

        Ok(())
    }

    pub fn gpio_write(&self, gpio: Gpio, level: GpioLevel) -> Result<()> {
        let err = unsafe { pigpiod_if2::gpio_write(self.0 .0, gpio.0, level as ffi::c_uint) };

        if err.is_negative() {
            return Err(Error::new(err));
        }

        Ok(())
    }

    fn file_open(&self, path: &path::Path, mode: FileMode) -> Result<Handle> {
        let filestr = ffi::CString::new(path.to_string_lossy().as_bytes())?;
        let pfile = filestr.as_ptr().cast_mut();

        let handle = unsafe { pigpiod_if2::file_open(self.0 .0, pfile, mode as ffi::c_uint) };

        if handle.is_negative() {
            return Err(Error::new(handle));
        }

        Ok(Handle(handle as ffi::c_uint))
    }

    fn file_read(&self, handle: &Handle, buf: &mut [u8]) -> Result<usize> {
        let pbuf = buf.as_mut_ptr().cast();
        let buflen = buf.len() as ffi::c_uint;

        let count = unsafe { pigpiod_if2::file_read(self.0 .0, handle.0, pbuf, buflen) };

        if count.is_negative() {
            return Err(Error::new(count));
        }

        Ok(count as usize)
    }

    fn file_close(&self, handle: &Handle) {
        unsafe { pigpiod_if2::file_close(self.0 .0, handle.0) };
    }
}

impl Drop for Init {
    fn drop(&mut self) {
        unsafe { pigpiod_if2::pigpio_stop(self.0) }
    }
}

#[derive(Debug)]
struct Handle(ffi::c_uint);
#[derive(Debug)]
pub struct File<'a> {
    pi: &'a Pi<Init>,
    handle: Handle,
}

#[derive(Debug, Clone, Copy)]
pub enum FileMode {
    Read = pigpiod_if2::PI_FILE_READ as isize,
    Write = pigpiod_if2::PI_FILE_WRITE as isize,
    RW = pigpiod_if2::PI_FILE_RW as isize,
}

impl<'a> File<'a> {
    pub fn open<P>(pi: &'a Pi<Init>, path: P, mode: FileMode) -> Result<Self>
    where
        P: AsRef<path::Path>,
    {
        let handle = pi.file_open(path.as_ref(), mode)?;
        Ok(Self { pi, handle })
    }

    pub fn close(self) {}
}

impl<'a> io::Read for File<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(self.pi.file_read(&self.handle, buf)?)
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.pi.file_close(&self.handle);
    }
}

pub fn read_to_string<P>(pi: &Pi<Init>, path: P) -> Result<String>
where
    P: AsRef<path::Path>,
{
    use io::Read;

    let mut file = File::open(pi, path, FileMode::Read)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
