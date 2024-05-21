use std::collections::LinkedList;

use crate::prelude::*;

const SPACE: u8 = 0xff;
const MINUS: u8 = 0xbf;
const DOT_MASK: u8 = 0x7f;
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

pub trait SegmentDisplay<const N: usize, T> {
    fn shift_register(&self) -> &ShiftRegister<N>;
    fn parse(value: T) -> Result<[u8; N]>;

    fn display(&self, value: T) -> Result<()> {
        let arr = Self::parse(value)?;

        self.shift_register().shift_n_bytes(arr)?;
        self.shift_register().save()?;

        Ok(())
    }
}

impl<const N: usize, T: ToString> SegmentDisplay<N, T> for ShiftRegister<N> {
    fn shift_register(&self) -> &ShiftRegister<N> {
        self
    }

    fn parse(value: T) -> Result<[u8; N]> {
        fn to_byte(value: char) -> Option<u8> {
            match value.to_digit(10) {
                Some(digit) => Some(DIGITS[digit as usize]),
                None => match value {
                    '-' => Some(MINUS),
                    _ => None,
                },
            }
        }

        let mut list: LinkedList<u8> = LinkedList::new();
        let mut current: Option<u8> = None;

        for next in value.to_string().chars() {
            if list.len() >= N {
                return Err(Error::other("number to long"));
            }

            match current {
                None => current = Some(to_byte(next).unwrap_or(SPACE)),
                Some(byte) => match next {
                    '.' => {
                        current = None;
                        list.push_back(byte & DOT_MASK);
                    }
                    next => {
                        current = Some(to_byte(next).unwrap_or(SPACE));
                        list.push_back(byte);
                    }
                },
            }
        }

        if let Some(current) = current {
            list.push_back(current)
        }

        while list.len() < N {
            list.push_front(SPACE)
        }

        if list.len() > N {
            return Err(Error::other("number to long"));
        }

        let mut iter = list.into_iter();

        Ok(std::array::from_fn(|_| iter.next().unwrap()))
    }
}
