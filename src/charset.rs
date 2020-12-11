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

pub fn parse_charset(string: &String) -> Result<ByteSet> {
    fn err_escape_hyphen() -> Result<ByteSet> {
        return Err(anyhow!("Hyphens must be escaped"));
    }
    fn err_invalid_escape(byte: u8) -> Result<ByteSet> {
        return Err(anyhow!("Invalid escape sequence: \"\\{}\"", byte as char));
    }
    let bytes = string.as_bytes();
    let invert = bytes[0] == CARET;
    let mut bytes = bytes.iter();
    if invert {
        bytes.next();
    }
    let mut state = Start;
    let mut result = ByteSet::new();
    for &byte in bytes {
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
        Escape | RangeEscape(_) => {
            return Err(anyhow!("Unterminated escape sequence"));
        },
        Range(_) => {
            return Err(anyhow!("Unterminated character range"));
        },
        _ => { },
    }
    if invert {
        let mut complement = ByteSet::from_raw_parts(TYPEABLE);
        complement.difference(&result);
        return Ok(complement);
    }
    if !result.is_subset(&ByteSet::from_raw_parts(TYPEABLE)) {
        return Err(anyhow!("Detected untypeable character"));
    }
    if result.is_empty() {
        return Err(anyhow!("Character set is empty"));
    }
    return Ok(result);
}
