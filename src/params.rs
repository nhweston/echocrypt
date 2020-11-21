use std::env;

use anyhow::*;

use crate::byte_set::ByteSet;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 126;

const DEFAULT_PWD_LEN: usize = 24;
const DEFAULT_NUM_PWDS: usize = 1;

const USAGE_OPTS: &str = r#"
    -e excluded_chars   exclude these characters from the charset
    -l password_length  generate passwords of this length (default 24)
    -n num_passwords    generate this many passwords (default 1)"#;

pub struct Params {
    pub cset: Vec<u8>,
    pub pwd_len: usize,
    pub num_pwds: usize,
}

impl Params {

    pub fn new(args: Vec<String>) -> Result<Params> {
        let mut iter = args.iter();
        iter.next();
        let mut excluded = ByteSet::new();
        let mut pwd_len = DEFAULT_PWD_LEN;
        let mut num_pwds = DEFAULT_NUM_PWDS;
        loop {
            match (iter.next().map(|s| s.as_str()), iter.next()) {
                (Some("-e"), Some(chars)) => {
                    for byte in chars.bytes() {
                        excluded.insert(byte);
                    }
                },
                (Some("-l"), Some(pwd_len_str)) => {
                    pwd_len = pwd_len_str.parse::<usize>()?;
                    if pwd_len == 0 {
                        return Err(anyhow!("Password length must not be zero"));
                    }
                },
                (Some("-n"), Some(num_pwds_str)) => {
                    num_pwds = num_pwds_str.parse::<usize>()?;
                    if num_pwds == 0 {
                        return Err(anyhow!("Number of passwords must not be zero"));
                    }
                },
                (Some(_), _) => {
                    return Err(anyhow!(usage()));
                },
                (None, _) => {
                    break;
                },
            }
        }
        let mut cset = Vec::new();
        for byte in ASCII_START..=ASCII_END {
            if !excluded.contains(byte) {
                cset.push(byte);
            }
        }
        if cset.is_empty() {
            return Err(anyhow!("Character set must not be empty"));
        }
        Ok(Params { cset, pwd_len, num_pwds })
    }

}

fn usage() -> String {
    let exec = env::args().next().unwrap_or("".to_string());
    format!("Usage: {} [options]{}", exec, USAGE_OPTS)
}
