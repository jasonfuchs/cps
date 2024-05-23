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

impl<const N: usize> FromStr for W<[u8; N]> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SPACE: u8 = 0xff;
        const MINUS: u8 = 0xbf;
        const DIGITS: [u8; 10] = [
            0b1100_0000,
            0b1111_1001,
            0b1010_0100,
            0b1011_0000,
            0b1001_1001,
            0b1001_0010,
            0b1000_0010,
            0b1111_1000,
            0b1000_0000,
            0b1001_0000,
        ];

        let mut chars = s.chars().peekable();
        let mut vec = Vec::<u8>::with_capacity(N);

        let mut iter = s
            .chars()
            .filter(|&c| c != '.')
            .map(|c| match c {
                '0' => 0b1100_0000,
                '1' => 0b1111_1001,
                '2' => 0b1010_0100,
                '3' => 0b1011_0000,
                '4' => 0b1001_1001,
                '5' => 0b1001_0010,
                '6' => 0b1000_0010,
                '7' => 0b1111_1000,
                '8' => 0b1000_0000,
                '9' => 0b1001_0000,
                '-' => 0xbf,
                _ => 0xff,
            })
            .map(|c| {
                if let Some(&'.') = chars.peek() {
                    chars.next();
                    vec.push(c & 0x7f)
                } else {
                    chars.next();
                    vec.push(c)
                }
            });

        todo!()
    }
}
