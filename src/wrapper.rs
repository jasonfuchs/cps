use std::ffi::CStr;

// The all important wrapper
//
// Using the newtype pattern
pub struct W<T>(pub T);

impl std::fmt::Display for W<&CStr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_bytes().escape_ascii())
    }
}
