use std::env;
use std::fs::File;
use std::io::Read;

use crate::Result;

const DEFAULT_BLK_LEN: usize = 256;
const DEFAULT_PWD_LEN: usize = 24;
const DEFAULT_NUM_PWDS: usize = 1;

pub struct Params {
    data: Vec<u8>,
    cset: Vec<u8>,
    blk_len: usize,
    pwd_len: usize,
    num_pwds: usize,
}

impl Params {

    pub fn new(args: Vec<String>) -> Result<Params> {
        let mut iter = args.iter();
        iter.next();
        let data_path = match iter.next() {
            Some(path) => Ok(path),
            None => Err(usage()),
        }?;
        let mut cset_path: Option<String> = None;
        let mut blk_len = DEFAULT_BLK_LEN;
        let mut pwd_len = DEFAULT_PWD_LEN;
        let mut num_pwds = DEFAULT_NUM_PWDS;
        loop {
            match iter.next().map(|s| s.as_str()) {
                Some("-b") => unimplemented!(),
                Some("-c") => unimplemented!(),
                Some("-l") => unimplemented!(),
                Some("-n") => unimplemented!(),
                Some(o) => break Err(usage()),
                None => break Ok(params),
            }
        }
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
    format!("Usage: {} length data_path [-c charset_path]", exec)
}
