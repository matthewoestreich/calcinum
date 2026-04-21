use crate::NumberError;
use std::{fmt, str::FromStr};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HexDigit {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
}

impl HexDigit {
    pub fn to_char(self, uppercase: bool) -> char {
        let value = self as u8;

        match value {
            0..=9 => (value + b'0') as char,
            10..=15 => {
                if uppercase {
                    (value - 10 + b'A') as char
                } else {
                    (value - 10 + b'a') as char
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn to_lowercase(self) -> String {
        self.to_char(false).to_string()
    }

    pub fn to_uppercase(self) -> String {
        self.to_char(true).to_string()
    }
}

impl TryFrom<u8> for HexDigit {
    type Error = NumberError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => HexDigit::D0,
            1 => HexDigit::D1,
            2 => HexDigit::D2,
            3 => HexDigit::D3,
            4 => HexDigit::D4,
            5 => HexDigit::D5,
            6 => HexDigit::D6,
            7 => HexDigit::D7,
            8 => HexDigit::D8,
            9 => HexDigit::D9,
            10 => HexDigit::A,
            11 => HexDigit::B,
            12 => HexDigit::C,
            13 => HexDigit::D,
            14 => HexDigit::E,
            15 => HexDigit::F,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("'{value}' out of hex range"),
                });
            }
        })
    }
}

impl TryFrom<char> for HexDigit {
    type Error = NumberError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let value = match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='F' => c as u8 - b'A' + 10,
            'a'..='f' => c as u8 - b'a' + 10,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("invalid hex char '{c}'"),
                });
            }
        };

        Ok(HexDigit::try_from(value).expect("already verified in range"))
    }
}

impl TryFrom<&char> for HexDigit {
    type Error = NumberError;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Self::try_from(*c)
    }
}

impl FromStr for HexDigit {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hc = match s {
            "0" => HexDigit::D0,
            "1" => HexDigit::D1,
            "2" => HexDigit::D2,
            "3" => HexDigit::D3,
            "4" => HexDigit::D4,
            "5" => HexDigit::D5,
            "6" => HexDigit::D6,
            "7" => HexDigit::D7,
            "8" => HexDigit::D8,
            "9" => HexDigit::D9,
            "10" => HexDigit::A,
            "11" => HexDigit::B,
            "12" => HexDigit::C,
            "13" => HexDigit::D,
            "14" => HexDigit::E,
            "15" => HexDigit::F,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("invalid hex character : '{s}'"),
                });
            }
        };
        Ok(hc)
    }
}

impl fmt::Display for HexDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = *self as u8;

        let c = match value {
            0..=9 => (value + b'0') as char,
            10..=15 => (value - 10 + b'A') as char,
            _ => unreachable!(),
        };

        write!(f, "{c}")
    }
}
