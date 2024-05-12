use std::ffi::c_uint;

use pigpiod_if2::*;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShiftRegister<const N: usize> {
    pi: Pi,
    ds: c_uint,
    sh_cp: c_uint,
    st_cp: c_uint,
}

impl<const N: usize> ShiftRegister<N> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn builder() -> ShiftRegisterBuilder<N, NoAddr, NoPort, NoDs, NoShCp, NoStCp> {
        ShiftRegisterBuilder::<N, NoAddr, NoPort, NoDs, NoShCp, NoStCp>::new()
    }

    pub fn get_ref(&self) -> &Pi {
        &self.pi
    }

    pub fn into_inner(self) -> Pi {
        self.pi
    }

    #[inline]
    fn strobe_high(&self, gpio: c_uint) -> Result<()> {
        self.pi.gpio_write(gpio, PI_LOW)?;
        self.pi.gpio_write(gpio, PI_HIGH)?;

        Ok(())
    }

    pub fn shift(&self) -> Result<()> {
        self.strobe_high(self.sh_cp)
    }

    pub fn save(&self) -> Result<()> {
        self.strobe_high(self.st_cp)
    }

    pub fn reset(&self) -> Result<()> {
        self.shift_n_bytes([0; N])
    }

    pub fn shift_byte(&self, value: u8) -> Result<()> {
        const MASK: u8 = 1;

        for i in (0..8).rev() {
            let bit = (value >> i) & MASK;

            self.pi.gpio_write(self.ds, bit.into())?;
            self.shift()?;
        }

        Ok(())
    }

    pub fn shift_n_bytes(&self, value: [u8; N]) -> Result<()> {
        for i in value {
            self.shift_byte(i)?;
        }

        Ok(())
    }
}

pub struct NoAddr;
pub struct NoPort;
pub struct NoDs;
pub struct NoShCp;
pub struct NoStCp;

#[derive(Debug)]
pub struct ShiftRegisterBuilder<const N: usize, Addr, Port, Ds, ShCp, StCp> {
    addr: Addr,
    port: Port,
    ds: Ds,
    sh_cp: ShCp,
    st_cp: StCp,
}

impl<const N: usize, Addr, Port, Ds, ShCp, StCp>
    ShiftRegisterBuilder<N, Addr, Port, Ds, ShCp, StCp>
{
    pub fn new() -> ShiftRegisterBuilder<N, NoAddr, NoPort, NoDs, NoShCp, NoStCp> {
        ShiftRegisterBuilder {
            addr: NoAddr,
            port: NoPort,
            ds: NoDs,
            sh_cp: NoShCp,
            st_cp: NoStCp,
        }
    }

    pub fn addr<'a, A>(self, addr: A) -> ShiftRegisterBuilder<N, &'a str, Port, Ds, ShCp, StCp>
    where
        A: Into<&'a str>,
    {
        self._addr(addr.into())
    }

    fn _addr<'a>(self, addr: &'a str) -> ShiftRegisterBuilder<N, &'a str, Port, Ds, ShCp, StCp> {
        let Self {
            port,
            ds,
            sh_cp,
            st_cp,
            ..
        } = self;

        ShiftRegisterBuilder {
            addr,
            port,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn port(self, port: u16) -> ShiftRegisterBuilder<N, Addr, u16, Ds, ShCp, StCp> {
        let Self {
            addr,
            ds,
            sh_cp,
            st_cp,
            ..
        } = self;

        ShiftRegisterBuilder {
            addr,
            port,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn ds(self, ds: c_uint) -> ShiftRegisterBuilder<N, Addr, Port, c_uint, ShCp, StCp> {
        let Self {
            addr,
            port,
            sh_cp,
            st_cp,
            ..
        } = self;

        ShiftRegisterBuilder {
            addr,
            port,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn sh_cp(self, sh_cp: c_uint) -> ShiftRegisterBuilder<N, Addr, Port, Ds, c_uint, StCp> {
        let Self {
            addr,
            port,
            ds,
            st_cp,
            ..
        } = self;

        ShiftRegisterBuilder {
            addr,
            port,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn st_cp(self, st_cp: c_uint) -> ShiftRegisterBuilder<N, Addr, Port, Ds, ShCp, c_uint> {
        let Self {
            addr,
            port,
            ds,
            sh_cp,
            ..
        } = self;

        ShiftRegisterBuilder {
            addr,
            port,
            ds,
            sh_cp,
            st_cp,
        }
    }
}

impl<'a, const N: usize> ShiftRegisterBuilder<N, &'a str, NoPort, c_uint, c_uint, c_uint> {
    pub fn build(self) -> Result<ShiftRegister<N>> {
        let Self {
            addr,
            ds,
            sh_cp,
            st_cp,
            ..
        } = self;

        let pi = Pi::new(addr)?;

        pi.set_mode(ds, pigpiod_if2::PI_INPUT)?;
        pi.set_mode(sh_cp, pigpiod_if2::PI_INPUT)?;
        pi.set_mode(st_cp, pigpiod_if2::PI_INPUT)?;

        Ok(ShiftRegister {
            pi,
            ds,
            sh_cp,
            st_cp,
        })
    }
}

impl<'a, const N: usize> ShiftRegisterBuilder<N, &'a str, u16, c_uint, c_uint, c_uint> {
    pub fn build(self) -> Result<ShiftRegister<N>> {
        let Self {
            addr,
            port,
            ds,
            sh_cp,
            st_cp,
        } = self;

        let pi = Pi::with_port(addr, port)?;

        pi.set_mode(ds, pigpiod_if2::PI_INPUT)?;
        pi.set_mode(sh_cp, pigpiod_if2::PI_INPUT)?;
        pi.set_mode(st_cp, pigpiod_if2::PI_INPUT)?;

        Ok(ShiftRegister {
            pi,
            ds,
            sh_cp,
            st_cp,
        })
    }
}
