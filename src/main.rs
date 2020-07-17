use core::result;
use std::env;
use std::fs::File;
use std::io::Read;

use crate::gen::generate;

mod gen;
mod bit_vec;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 126;

type Result<T> = result::Result<T, String>;

fn main() {
    let args = env::args().collect();
    match run(args) {
        Ok(result) => println!("{}", result),
        Err(msg) => eprintln!("{}", msg)
    }
}

fn run(args: Vec<String>) -> Result<String> {
    let len = match args.get(1) {
        Some(len_str) => len_str.parse::<usize>().map_err(|e| e.to_string()),
        None => Err(usage())
    }?;
    let data = match args.get(2) {
        Some(path) => load_file(path),
        None => Err(usage())
    }?;
    let charset = match (args.get(3), args.get(4)) {
        (Some(opt), Some(charset_path)) =>
            if opt == "-c" { Ok(load_file(charset_path)?) }
            else { Err(usage()) },
        (None, None) => {
            let mut charset = Vec::new();
            for byte in ASCII_START..=ASCII_END { charset.push(byte); }
            Ok(charset)
        },
        _ => Err(usage())
    }?;
    let bytes = generate(&data, &charset, len);
    let result = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    Ok(result)
}

fn usage() -> String {
    let exec = env::args().next().unwrap_or("".to_string());
    format!("Usage: {} length data_path [-c charset_path]", exec)
}

fn load_file(path: &String) -> Result<Vec<u8>> {
    let mut data = Vec::<u8>::new();
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    match file.read_to_end(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e.to_string())
    }
}
