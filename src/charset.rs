use anyhow::*;

use State::*;

use crate::byte_set::ByteSet;

const HYPHEN: u8 = 45;
const BACKSLASH: u8 = 92;
const CARET: u8 = 94;

pub const TYPEABLE: [u64; 4] = [
    0b1111_1111_1111_1111__1111_1111_1111_1111__0000_0000_0000_0000__0000_0000_0000_0000,
    0b0111_1111_1111_1111__1111_1111_1111_1111__1111_1111_1111_1111__1111_1111_1111_1111,
    0,
    0,
];

enum State {
    Start,
    Char(u8),
    Escape,
    Range(u8),
    RangeEscape(u8),
}

pub fn default_charset() -> Vec<u8> {
    ByteSet::from_raw_parts(TYPEABLE).into()
}

pub fn parse_charset_spec(string: &String) -> Result<Vec<u8>> {
    fn err_escape_hyphen() -> Result<Vec<u8>> {
        Err(anyhow!("Hyphens must be escaped"))
    }
    fn err_invalid_escape(byte: u8) -> Result<Vec<u8>> {
        Err(anyhow!("Invalid escape sequence: \"\\{}\"", byte as char))
    }
    if string.is_empty() {
        return Err(anyhow!("Empty charset specification"));
    }
    let bytes = string.as_bytes();
    let invert = bytes[0] == CARET;
    let mut bytes = bytes.iter();
    if invert {
        bytes.next();
    }
    let mut state = Start;
    let mut result = ByteSet::new();
    let typeable = ByteSet::from_raw_parts(TYPEABLE);
    for &byte in bytes {
        if !typeable.contains(byte) {
            return Err(anyhow!("Found untypeable or non-ASCII character"));
        }
        match (state, byte) {
            (Start, HYPHEN) => {
                return err_escape_hyphen();
            },
            (Start, BACKSLASH) => {
                state = Escape;
            },
            (Start, byte) => {
                result.insert(byte);
                state = Char(byte);
            },
            (Char(prev), HYPHEN) => {
                state = Range(prev);
            },
            (Char(_), BACKSLASH) => {
                state = Escape;
            },
            (Char(_), byte) => {
                result.insert(byte);
                state = Char(byte);
            },
            (Escape, byte) => {
                if byte == HYPHEN || byte == BACKSLASH {
                    result.insert(byte);
                    state = Char(byte);
                }
                else {
                    return err_invalid_escape(byte);
                }
            },
            (Range(_), HYPHEN) => {
                return err_escape_hyphen();
            },
            (Range(start), BACKSLASH) => {
                state = RangeEscape(start);
            },
            (Range(start), end) => {
                for byte in (start + 1)..=end {
                    result.insert(byte);
                }
                state = Start;
            },
            (RangeEscape(start), byte) => {
                if byte == HYPHEN || byte == BACKSLASH {
                    for byte in (start + 1)..=byte {
                        result.insert(byte);
                    }
                    state = Start;
                }
                else {
                    return err_invalid_escape(byte);
                }
            },
        }
    }
    match state {
        Escape | RangeEscape(_) => Err(anyhow!("Unterminated escape sequence")),
        Range(_) => Err(anyhow!("Unterminated character range")),
        _ => {
            if invert {
                let tmp = result;
                result = typeable;
                result.difference(&tmp);
            }
            if result.is_empty() { Err(anyhow!("Character set is empty")) }
            else { Ok(result.into()) }
        },
    }
}
