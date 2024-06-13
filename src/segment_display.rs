use std::error;
use std::result;

use crate::prelude::*;

pub trait SegemntDisplay<'a, const N: usize, T> {
    type Error: error::Error + Send + Sync + 'static;

    fn shift_register(&self) -> &ShiftRegister<'a, N>;
    fn parse(value: T) -> result::Result<[u8; N], Self::Error>;

    fn display(&self, value: T) -> Result<()> {
        let shift_reg = self.shift_register();
        let arr = Self::parse(value).map_err(Error::other)?;

        shift_reg.push_n(&arr)?;

        Ok(())
    }
}
