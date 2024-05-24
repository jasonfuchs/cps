use std::{ffi::CStr, fmt, str::FromStr};

use diesel::serialize::ToSql;

// The all important wrapper
//
// Using the newtype pattern
pub struct W<T>(pub T);

pub trait Wrap: Sized {
    fn wrap(self) -> W<Self>;
}

impl<T: Sized> Wrap for T {
    fn wrap(self) -> W<Self> {
        W(self)
    }
}

impl fmt::Display for W<&CStr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_bytes().escape_ascii())
    }
}
