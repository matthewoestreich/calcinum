//! # Formatting
//!
//! - We use the same spec as the cli, without the need to start with `:`.
//! - You need to provide a string to the `.format("..")`method using the following
//!   grammar and syntax as described in the docs found at the top of `src/lib.rs` or
//!   here [Formatting Help](https://docs.rs/calcinum/latest/calcinum/index.html#cli-formatting).
//!
//! # Spec
//!
//! At a high level:
//!
//! ```text
//! 0999b8
//! | | ||
//! | | |+--   (8) GROUPING : Provide a number and we will group your output by `N` characters.
//! | | +---   (b) KIND : This is the format you want, e.g., binary, hex, base64, etc ...
//! | +----- (999) WIDTH : How many characters do you want your output to be.
//! +-------   (0) ZERO PAD : Do you want us to pad width with 0's? If not provided we pad with spaces.
//! ```
//!
//! # Examples
//!
//! ```rust
//! use calcinum::Number;
//!
//! let n = Number::from(123);
//!
//! // Format number as binary.
//! n.format("b"); // "1111011"
//!
//! // Format number as binary with a width of 12, non zero padded.
//! n.format("12b"); // "     1111011"
//!
//! // Format number as binary with a width o 12, zero padded.
//! n.format("012b"); // "000001111011"
//!
//! // Format number as binary width a width of 12, zero padded, groups of 4.
//! n.format("012b4"); // "0000 0111 1011"
//! ```
//!

use crate::Number;
use std::fmt;
use varienum::VariantsVec;

#[derive(Debug)]
pub enum State {
    Start,
    Width,
    ZeroPad,
    Scale,
    Kind,
    Group,
}

#[derive(Debug, Default, PartialEq, Eq, VariantsVec)]
pub enum Kind {
    #[description = "b (binary)"]
    Binary,
    #[description = "X (hex upper)"]
    HexadecimalUpper,
    #[description = "x (hex lower"]
    HexadecimalLower,
    #[description = "B (base64)"]
    Base64,
    #[description = "N (Number)"]
    Number,
    #[default]
    Null,
}

impl Kind {
    pub fn is_null(&self) -> bool {
        matches!(self, Kind::Null)
    }
}

impl From<char> for Kind {
    fn from(c: char) -> Self {
        match c {
            'b' => Self::Binary,
            'X' => Self::HexadecimalUpper,
            'x' => Self::HexadecimalLower,
            'B' => Self::Base64,
            'N' => Self::Number,
            _ => Self::Null,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Binary => write!(f, "b"),
            Kind::HexadecimalUpper => write!(f, "X"),
            Kind::HexadecimalLower => write!(f, "x"),
            Kind::Base64 => write!(f, "B"),
            Kind::Number => write!(f, "N"),
            Kind::Null => write!(f, ""),
        }
    }
}

#[derive(Default, Debug)]
pub struct FormatSpec {
    zero_pad: bool,
    width: Option<usize>,
    scale: Option<usize>,
    // the only part of the spec that is required.
    kind: Kind,
    group: Option<usize>,
}

impl FormatSpec {
    pub fn parse(spec: &str) -> Result<FormatSpec, String> {
        let mut zero_pad = false;
        let mut width = String::new();
        let mut group = String::new();
        let mut scale = String::new();
        let mut kind = Kind::Null;
        let mut state = State::Start;

        for c in spec.chars() {
            match state {
                State::Start => match c {
                    '0' => {
                        zero_pad = true;
                        state = State::ZeroPad;
                    }
                    c if c.is_ascii_digit() => {
                        width.push(c);
                        state = State::Width;
                    }
                    c if c.is_ascii_alphabetic() => {
                        kind = Kind::from(c);
                        state = State::Kind;
                    }
                    _ => return Err(format!("unexpected char '{c}' in Start")),
                },
                State::ZeroPad => match c {
                    c if c.is_ascii_digit() => {
                        width.push(c);
                        state = State::Width;
                    }
                    c if c.is_alphabetic() => {
                        kind = Kind::from(c);
                        state = State::Kind;
                    }
                    _ => return Err(format!("unexpected char '{c}' after Start")),
                },
                State::Width => match c {
                    '.' => {
                        state = State::Scale;
                    }
                    c if c.is_ascii_digit() => {
                        width.push(c);
                        state = State::Width;
                    }
                    c if c.is_ascii_alphabetic() => {
                        kind = Kind::from(c);
                        state = State::Kind;
                    }
                    _ => return Err(format!("unexxpected char '{c}' in Width")),
                },
                State::Kind => match c {
                    c if c.is_ascii_digit() => {
                        group.push(c);
                        state = State::Group;
                    }
                    _ => return Err(format!("unexpected char '{c}' in Kind")),
                },
                State::Scale => match c {
                    c if c.is_ascii_digit() => {
                        scale.push(c);
                    }
                    _ => return Err(format!("unexpect char '{c}' in Scale")),
                },
                State::Group => match c {
                    c if c.is_ascii_digit() => group.push(c),
                    _ => return Err(format!("unexpected char '{c}' in Group")),
                },
            }
        }

        if kind.is_null() {
            return Err("kind is required!".to_string());
        };

        let width = if width.is_empty() {
            None
        } else {
            width
                .parse()
                .map(Some)
                .map_err(|e| format!("unable to parse width : {e:?}"))?
        };

        let group = if group.is_empty() {
            None
        } else {
            group
                .parse()
                .map(Some)
                .map_err(|e| format!("unable to parse group size : {e:?}"))?
        };

        let scale = if scale.is_empty() {
            None
        } else {
            scale
                .parse()
                .map(Some)
                .map_err(|e| format!("unable to parse scale : {e:?} "))?
        };

        Ok(Self {
            zero_pad,
            width,
            scale,
            kind,
            group,
        })
    }
}

pub struct Formatter;

impl Formatter {
    pub fn format_number(number: &Number, spec: FormatSpec) -> Result<String, String> {
        let num_str = match spec.kind {
            Kind::Number => number.to_string(),
            Kind::Binary => number.to_binary_str(),
            Kind::HexadecimalLower => number.to_hexadecimal_str(false),
            Kind::HexadecimalUpper => number.to_hexadecimal_str(true),
            Kind::Base64 => number.to_base64_str(),
            Kind::Null => return Err(format!("unrecognized type '{:?}'", spec.kind)),
        };

        let (is_negative, num_str) = match num_str.strip_prefix('-') {
            Some(rest) => (true, rest.to_string()),
            None => (false, num_str),
        };

        println!("spec.Kind == {}", spec.kind);

        if spec.kind == Kind::Number {
            println!("spec.kind is N and we are in 'if' statement.");
            if !number.is_decimal() {
                return Ok(num_str);
            }
            let Some(scale) = spec.scale else {
                return Ok(num_str);
            };
            let (int_part, fract_part) = num_str.split_once('.').unwrap_or((&num_str, ""));
            let fmted_fract_part: String = fract_part.chars().take(scale).collect();
            println!("scale='{scale}' | int_part='{int_part}' | fractt_part='{fract_part}'");
            return Ok(format!("{int_part}.{fmted_fract_part}"));
        }

        let pad_char = if spec.zero_pad { '0' } else { ' ' };

        let mut group_pad = 0;
        let mut width_pad = 0;

        if let Some(w) = spec.width {
            width_pad = w.saturating_sub(num_str.len());
        }
        if let Some(group) = spec.group {
            let min_len = num_str.len() + width_pad;
            group_pad = Self::next_multiple(group, min_len) - min_len;
        }

        let cap = width_pad + group_pad + num_str.len() + if is_negative { 1 } else { 0 };
        let mut num_fmtd = String::with_capacity(cap);

        // Base64 already encoded the negative symbol.
        // Only add negative sign to start of string if we are padding with zeros,
        // otherwise the final result looks like `"-     0101010101"`
        if is_negative && pad_char == '0' && spec.kind != Kind::Base64 {
            num_fmtd.push('-');
        }

        for _ in 0..width_pad {
            num_fmtd.push(pad_char);
        }
        for _ in 0..group_pad {
            num_fmtd.push('0');
        }

        // Base64 already encoded the negative symbol.
        // Only add negative sign after padding if padding char == ' ',
        // otherwise the final result looks like `"-     0101010101"`
        if is_negative && pad_char == ' ' && spec.kind != Kind::Base64 {
            num_fmtd.push('-');
        }

        // Now we have padding in our 'formatted' string,
        // push our converted Number string into it.
        num_fmtd.push_str(&num_str);

        if let Some(group) = spec.group {
            let mut s = String::with_capacity(num_fmtd.len());
            let mut i = 0;
            for c in num_fmtd.chars() {
                if c == '-' {
                    s.push(c);
                    continue;
                }
                if i != 0 && i % group == 0 {
                    s.push(' ');
                }
                s.push(c);
                i += 1;
            }
            num_fmtd = s;
        }

        Ok(num_fmtd)
    }

    /// Finds the next multiple, `m`,  starting at `n`.
    /// If `n` is already a multiple of `m`, we return `n`.
    /// If `m` or `n` are 0, we return 0.
    fn next_multiple(m: usize, n: usize) -> usize {
        if m == 0 || n == 0 {
            return 0;
        }
        if n.is_multiple_of(m) {
            return n;
        };
        ((n / m) + 1) * m
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use std::mem;

    #[rstest]
    #[case::fmt_spec1("8b4", false, Some(8), Kind::Binary, Some(4))]
    #[case::fmt_spec2("0x4", true, None, Kind::HexadecimalLower, Some(4))]
    #[case::fmt_spec3("16x2", false, Some(16), Kind::HexadecimalLower, Some(2))]
    #[case::fmt_spec4("b", false, None, Kind::Binary, None)]
    #[case::fmt_spec5("08b4", true, Some(8), Kind::Binary, Some(4))]
    #[case::fmt_spec6("0b", true, None, Kind::Binary, None)]
    #[should_panic]
    #[case::fmt_spec7("z", true, None, Kind::Null, None)]
    fn format_spec(
        #[case] spec_str: &str,
        #[case] expected_zero_pad: bool,
        #[case] expected_width: Option<usize>,
        #[case] expected_kind: Kind,
        #[case] expected_group: Option<usize>,
    ) {
        let parsed = FormatSpec::parse(spec_str).unwrap();
        assert_eq!(parsed.zero_pad, expected_zero_pad);
        assert_eq!(parsed.width, expected_width);
        assert_eq!(parsed.group, expected_group);
        assert_eq!(
            mem::discriminant(&parsed.kind),
            mem::discriminant(&expected_kind),
            "expected kind '{expected_kind:?}' got kind '{:?}'",
            parsed.kind
        );
    }
}
