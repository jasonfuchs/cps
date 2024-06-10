use std::{ffi::CStr, fmt};

/// The all important wrapper
///
/// Using new type pattern to add methods and traits to foriegn types
pub struct W<T>(pub T);

impl<T> From<T> for W<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

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
