use crate::prelude::*;

#[derive(Debug)]
pub enum SegmentCode {
    Just(char),
    WithDot(char),
}

impl SegmentCode {
    pub fn new(chars: (char, char)) -> Option<Self> {
        match chars {
            ('.', '.') => Some(Self::Just('.')),
            ('.', _) => None,
            (c, '.') => Some(Self::WithDot(c)),
            (c, _) => Some(Self::Just(c)),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::WithDot(c) => Self::Just(*c).to_u8() & 0b0111_1111,
            Self::Just(c) => {
                let c = c.to_ascii_uppercase();
                let ascii = c as u8;
                if c.is_ascii_digit() {
                    let i = ascii - ('0' as u8);
                    NUMERALS[i as usize]
                } else if c.is_ascii_alphabetic() {
                    let i = ascii - ('A' as u8);
                    LETTERS[i as usize]
                } else {
                    match c {
                        ' ' => 0b1111_1111,
                        '-' => 0b1011_1111,
                        '_' => 0b1111_0111,
                        _ => 0b1111_1111,
                    }
                }
            }
        }
    }
}

static NUMERALS: [u8; 10] = [
    //.GFE_DCBA
    0b1100_0000, // 0
    0b1111_1001, // 1
    0b1010_0100, // 2
    0b1011_0000, // 3
    0b1001_1001, // 4
    0b1001_0010, // 5
    0b1000_0010, // 6
    0b1111_1000, // 7
    0b1000_0000, // 8
    0b1001_1000, // 9
];

static LETTERS: [u8; 26] = [
    0b1000_1000, // A
    0b1000_0011, // B
    0b1100_0110, // C
    0b1010_0001, // D
    0b1000_0110, // E
    0b1000_1110, // F
    0b1100_0010, // G
    0b1000_1001, // H
    0b1100_1111, // I
    0b1110_0001, // J
    0b1000_1010, // K
    0b1100_0111, // L
    0b1110_1010, // M
    0b1100_1000, // N
    0b1100_0000, // O
    0b1000_1100, // P
    0b1001_0100, // Q
    0b1100_1100, // R
    0b1001_0010, // S
    0b1000_0111, // T
    0b1100_0001, // U
    0b1100_0001, // V
    0b1101_0101, // W
    0b1000_1001, // X
    0b1001_0001, // Y
    0b1010_0100, // Z
];

pub trait SegmentDisplay<'a, const N: usize, T> {
    fn shift_register(&self) -> &ShiftRegister<'a, N>;
    fn parse(value: T) -> Result<[u8; N]>;

    fn write(&self, value: T) -> Result<()> {
        self.shift_register().push_arr(Self::parse(value)?)?;
        Ok(())
    }
}
