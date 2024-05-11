use std::ffi::c_uint;

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

    pub fn port<P>(self, port: P) -> ShiftRegisterBuilder<N, Addr, u16, Ds, ShCp, StCp>
    where
        P: Into<u16>,
    {
        self._port(port.into())
    }

    fn _port(self, port: u16) -> ShiftRegisterBuilder<N, Addr, u16, Ds, ShCp, StCp> {
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

    pub fn ds<D>(self, ds: D) -> ShiftRegisterBuilder<N, Addr, Port, c_uint, ShCp, StCp>
    where
        D: Into<c_uint>,
    {
        self._ds(ds.into())
    }

    fn _ds(self, ds: c_uint) -> ShiftRegisterBuilder<N, Addr, Port, c_uint, ShCp, StCp> {
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

    pub fn sh_cp<S>(self, sh_cp: S) -> ShiftRegisterBuilder<N, Addr, Port, Ds, c_uint, StCp>
    where
        S: Into<c_uint>,
    {
        self._sh_cp(sh_cp.into())
    }

    fn _sh_cp(self, sh_cp: c_uint) -> ShiftRegisterBuilder<N, Addr, Port, Ds, c_uint, StCp> {
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

    pub fn st_cp<S>(self, st_cp: S) -> ShiftRegisterBuilder<N, Addr, Port, Ds, ShCp, c_uint>
    where
        S: Into<c_uint>,
    {
        self._st_cp(st_cp.into())
    }

    fn _st_cp(self, st_cp: c_uint) -> ShiftRegisterBuilder<N, Addr, Port, Ds, ShCp, c_uint> {
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

        Ok(ShiftRegister {
            pi,
            ds,
            sh_cp,
            st_cp,
        })
    }
}
