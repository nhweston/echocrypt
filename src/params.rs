use std::env;
use std::fs::File;
use std::io::Read;

use crate::Result;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 126;

const DEFAULT_BLK_LEN: usize = 256;
const DEFAULT_PWD_LEN: usize = 24;
const DEFAULT_NUM_PWDS: usize = 1;

const USAGE_OPTS: &str = r#"
    -b block_length     aggregate this many bytes in each block (default 256)
    -c charset_file     use the charset specified by the given file
    -l password_length  generate passwords of this length (default 24)
    -n num_passwords    generate this many passwords"#;

pub struct Params {
    pub cset: Vec<u8>,
    pub blk_len: usize,
    pub pwd_len: usize,
    pub num_pwds: usize,
}

impl Params {

    pub fn new(args: Vec<String>) -> Result<Params> {
        let mut iter = args.iter();
        iter.next();
        let mut cset_path: Option<String> = None;
        let mut blk_len = DEFAULT_BLK_LEN;
        let mut pwd_len = DEFAULT_PWD_LEN;
        let mut num_pwds = DEFAULT_NUM_PWDS;
        loop {
            match (iter.next().map(|s| s.as_str()), iter.next()) {
                (Some("-b"), Some(blk_len_str)) => {
                    blk_len = blk_len_str.parse::<usize>().map_err(|e| e.to_string())?;
                },
                (Some("-c"), Some(path)) => {
                    cset_path = Some(path.clone());
                },
                (Some("-l"), Some(pwd_len_str)) => {
                    pwd_len = pwd_len_str.parse::<usize>().map_err(|e| e.to_string())?;
                },
                (Some("-n"), Some(num_pwds_str)) => {
                    num_pwds = num_pwds_str.parse::<usize>().map_err(|e| e.to_string())?;
                },
                (Some(_), _) => {
                    return Err(usage());
                },
                (None, _) => {
                    break;
                },
            }
        }
        let cset = match cset_path {
            Some(path) => load_file(&path)?,
            None => {
                let mut cset = Vec::<u8>::new();
                for byte in ASCII_START..=ASCII_END {
                    cset.push(byte);
                }
                cset
            },
        };
        Ok(Params { cset, blk_len, pwd_len, num_pwds })
    }

}

fn load_file(path: &String) -> Result<Vec<u8>> {
    let mut data = Vec::<u8>::new();
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    match file.read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e.to_string())
    }
}

fn usage() -> String {
    let exec = env::args().next().unwrap_or("".to_string());
    format!("Usage: {} [options]{}", exec, USAGE_OPTS)
}
