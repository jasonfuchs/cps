use std::ffi::CStr;

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

impl std::fmt::Display for W<&CStr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_bytes().escape_ascii())
    }
}
