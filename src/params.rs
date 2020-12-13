use std::env;

use anyhow::*;

use crate::charset::*;

const DEFAULT_PWD_LEN: usize = 24;
const DEFAULT_NUM_PWDS: usize = 1;

const USAGE_OPTS: &str = r#"
    -c charset          use this character set
    -l password_length  generate passwords of this length (default 24)
    -n num_passwords    generate this many passwords (default 1)"#;

pub struct Params {
    pub charset: Vec<u8>,
    pub pwd_len: usize,
    pub num_pwds: usize,
}

impl Params {

    pub fn new(args: Vec<String>) -> Result<Params> {
        let mut iter = args.iter();
        iter.next();
        let mut charset = None;
        let mut pwd_len = DEFAULT_PWD_LEN;
        let mut num_pwds = DEFAULT_NUM_PWDS;
        loop {
            match (iter.next().map(|s| s.as_str()), iter.next()) {
                (Some("-c"), Some(charset_spec)) => {
                    charset = Some(parse_charset_spec(charset_spec)?);
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
        let charset = charset.unwrap_or_else(|| default_charset());
        Ok(Params { charset, pwd_len, num_pwds })
    }

}

fn usage() -> String {
    let exec = env::args().next().unwrap_or("".to_string());
    format!("Usage: {} [options]{}", exec, USAGE_OPTS)
}
