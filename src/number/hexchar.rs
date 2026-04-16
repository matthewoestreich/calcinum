use crate::NumberError;
use std::{fmt, str::FromStr};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
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
    /// The argument must be a valid hexadecimal character.
    ///
    /// Valid characters are:
    /// - `'0'..='9'`
    /// - `'A'..='F'` (case-insensitive)
    pub fn from_char(hex: &char) -> Result<Self, NumberError> {
        Ok(match hex {
            '0' => Self::D0,
            '1' => Self::D1,
            '2' => Self::D2,
            '3' => Self::D3,
            '4' => Self::D4,
            '5' => Self::D5,
            '6' => Self::D6,
            '7' => Self::D7,
            '8' => Self::D8,
            '9' => Self::D9,
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("hex char '{hex}' is not a Nibble"),
                });
            }
        })
    }

    /// This method will return an error if an invalid hex character is encountered.
    ///
    /// Valid characters are:
    /// - `'0'..='9'`
    /// - `'A'..='F'` (case-insensitive)
    ///
    /// # Panics!
    ///
    /// - If an invalid hex character is encountered.
    pub fn from_str_unchecked(s: &str) -> Self {
        Self::from_str(s).expect("this method is unchecked")
    }

    /// The argument must be a valid hexadecimal character.
    ///
    /// Valid characters are:
    /// - `'0'..='9'`
    /// - `'A'..='F'` (case-insensitive)
    ///
    /// [!WARNING]  `panic!`
    /// If the argument is not a valid hex character.
    pub fn from_char_unchecked(hex: &char) -> Self {
        Self::from_char(hex).expect("this method is unchecked")
    }

    /// Returns `true` if `c` is a valid hexadecimal character, `false` if not.
    pub fn is_valid(c: &char) -> bool {
        matches!(
            c,
            '0' | '1'
                | '2'
                | '3'
                | '4'
                | '5'
                | '6'
                | '7'
                | '8'
                | '9'
                | 'A'
                | 'a'
                | 'B'
                | 'b'
                | 'C'
                | 'c'
                | 'D'
                | 'd'
                | 'E'
                | 'e'
                | 'F'
                | 'f'
        )
    }

    /// Converts `self` into it's hexadecimal character representation, as a `String`.
    pub fn to_str(self, uppercase: bool) -> String {
        let s = match self {
            HexChar::A => "a",
            HexChar::B => "b",
            HexChar::C => "c",
            HexChar::D => "d",
            HexChar::E => "e",
            HexChar::F => "f",
            _ => &format!("{self}"),
        };
        if uppercase {
            s.to_uppercase()
        } else {
            s.to_lowercase()
        }
    }
}

impl FromStr for HexChar {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::D0,
            "1" => Self::D1,
            "2" => Self::D2,
            "3" => Self::D3,
            "4" => Self::D4,
            "5" => Self::D5,
            "6" => Self::D6,
            "7" => Self::D7,
            "8" => Self::D8,
            "9" => Self::D9,
            "10" => Self::A,
            "11" => Self::B,
            "12" => Self::C,
            "13" => Self::D,
            "14" => Self::E,
            "15" => Self::F,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("'{s}' out of Nibble range 0..=15"),
                });
            }
        })
    }
}

impl fmt::Display for HexChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from(match self {
            HexChar::D0 => "0",
            HexChar::D1 => "1",
            HexChar::D2 => "2",
            HexChar::D3 => "3",
            HexChar::D4 => "4",
            HexChar::D5 => "5",
            HexChar::D6 => "6",
            HexChar::D7 => "7",
            HexChar::D8 => "8",
            HexChar::D9 => "9",
            HexChar::A => "10",
            HexChar::B => "11",
            HexChar::C => "12",
            HexChar::D => "13",
            HexChar::E => "14",
            HexChar::F => "15",
        });
        write!(f, "{s}")
    }
}
