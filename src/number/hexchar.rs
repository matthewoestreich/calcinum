use crate::NumberError;
use std::{fmt, str::FromStr};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HexChar {
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

impl HexChar {
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

impl TryFrom<u8> for HexChar {
    type Error = NumberError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => HexChar::D0,
            1 => HexChar::D1,
            2 => HexChar::D2,
            3 => HexChar::D3,
            4 => HexChar::D4,
            5 => HexChar::D5,
            6 => HexChar::D6,
            7 => HexChar::D7,
            8 => HexChar::D8,
            9 => HexChar::D9,
            10 => HexChar::A,
            11 => HexChar::B,
            12 => HexChar::C,
            13 => HexChar::D,
            14 => HexChar::E,
            15 => HexChar::F,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("'{value}' out of hex range"),
                });
            }
        })
    }
}

impl TryFrom<char> for HexChar {
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

        Ok(HexChar::try_from(value).expect("already verified in range"))
    }
}

impl TryFrom<&char> for HexChar {
    type Error = NumberError;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Self::try_from(*c)
    }
}

impl FromStr for HexChar {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hc = match s {
            "0" => HexChar::D0,
            "1" => HexChar::D1,
            "2" => HexChar::D2,
            "3" => HexChar::D3,
            "4" => HexChar::D4,
            "5" => HexChar::D5,
            "6" => HexChar::D6,
            "7" => HexChar::D7,
            "8" => HexChar::D8,
            "9" => HexChar::D9,
            "10" => HexChar::A,
            "11" => HexChar::B,
            "12" => HexChar::C,
            "13" => HexChar::D,
            "14" => HexChar::E,
            "15" => HexChar::F,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("invalid hex character : '{s}'"),
                });
            }
        };
        Ok(hc)
    }
}

impl fmt::Display for HexChar {
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
