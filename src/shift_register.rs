use crate::prelude::*;

#[derive(Debug)]
pub struct ShiftRegister<'a, const N: usize> {
    pi: &'a Pi<Init>,
    ds: Gpio,
    sh_cp: Gpio,
    st_cp: Gpio,
}

impl<'a, const N: usize> ShiftRegister<'a, N> {
    pub fn builder() -> ShiftRegisterBuilder<N, NoPi, NoDs, NoShCp, NoStCp> {
        ShiftRegisterBuilder {
            pi: NoPi,
            ds: NoDs,
            sh_cp: NoShCp,
            st_cp: NoStCp,
        }
    }

    pub fn strobe(&self, gpio: Gpio) -> Result<()> {
        self.pi.gpio_write(gpio, GpioLevel::Low)?;
        self.pi.gpio_write(gpio, GpioLevel::High)?;
        Ok(())
    }

    pub fn shift(&self) -> Result<()> {
        self.strobe(self.sh_cp)?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        self.strobe(self.st_cp)?;
        Ok(())
    }

    pub fn push(&self, byte: u8) -> Result<()> {
        const MASK: u8 = 1;

        for i in (0..8).rev() {
            let level = if (byte >> i) & MASK == 1 {
                GpioLevel::High
            } else {
                GpioLevel::Low
            };

            self.pi.gpio_write(self.ds, level)?;
            self.shift()?;
        }

        Ok(())
    }

    pub fn push_n(&self, bytes: &[u8; N]) -> Result<()> {
        for byte in bytes {
            self.push(*byte)?;
        }

        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.push_n(&[0; N])?;
        Ok(())
    }
}

pub struct NoPi;
pub struct NoDs;
pub struct NoShCp;
pub struct NoStCp;
#[derive(Debug)]
pub struct ShiftRegisterBuilder<const N: usize, T, U, S, L> {
    pi: T,
    ds: U,
    sh_cp: S,
    st_cp: L,
}

impl<const N: usize, T, U, V, W> ShiftRegisterBuilder<N, T, U, V, W> {
    pub fn pi(self, pi: &Pi<Init>) -> ShiftRegisterBuilder<N, &Pi<Init>, U, V, W> {
        let Self {
            ds, sh_cp, st_cp, ..
        } = self;
        ShiftRegisterBuilder {
            pi,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn ds(self, ds: Gpio) -> ShiftRegisterBuilder<N, T, Gpio, V, W> {
        let Self {
            pi, sh_cp, st_cp, ..
        } = self;
        ShiftRegisterBuilder {
            pi,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn sh_cp(self, sh_cp: Gpio) -> ShiftRegisterBuilder<N, T, U, Gpio, W> {
        let Self { pi, ds, st_cp, .. } = self;
        ShiftRegisterBuilder {
            pi,
            ds,
            sh_cp,
            st_cp,
        }
    }

    pub fn st_cp(self, st_cp: Gpio) -> ShiftRegisterBuilder<N, T, U, V, Gpio> {
        let Self { pi, ds, sh_cp, .. } = self;
        ShiftRegisterBuilder {
            pi,
            ds,
            sh_cp,
            st_cp,
        }
    }
}

impl<'a, const N: usize> ShiftRegisterBuilder<N, &'a Pi<Init>, Gpio, Gpio, Gpio> {
    pub fn build(self) -> Result<ShiftRegister<'a, N>> {
        let Self {
            pi,
            ds,
            sh_cp,
            st_cp,
        } = self;
        pi.set_mode(ds, GpioMode::Output)?;
        pi.set_mode(sh_cp, GpioMode::Output)?;
        pi.set_mode(st_cp, GpioMode::Output)?;
        Ok(ShiftRegister {
            pi,
            ds,
            sh_cp,
            st_cp,
        })
    }
}
