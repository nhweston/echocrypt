use cpal::{Device, Stream, StreamConfig};
use cpal::traits::DeviceTrait;

use crate::{Result, State};

pub trait Sample where Self: Sized + cpal::Sample {
    fn aggregate_bits(self) -> u8;
}

impl Sample for i16 {
    fn aggregate_bits(self) -> u8 {
        (self as u16).aggregate_bits()
    }
}

impl Sample for u16 {
    fn aggregate_bits(self) -> u8 {
        let mut val = self;
        let mut res = val;
        for _ in 1..16 {
            val <<= 1;
            res ^= val;
        }
        res as u8
    }
}

impl Sample for f32 {
    fn aggregate_bits(self) -> u8 {
        let mut val = u32::from_le_bytes(self.to_ne_bytes());
        let mut res = val;
        for _ in 1..32 {
            val >>= 1;
            res ^= val;
        }
        res as u8
    }
}

pub fn stream<T: Sample>(
    dev: Device,
    conf: StreamConfig,
    state: State,
) -> Result<Stream> {
    let State { mut gen, mut mutex, mut num_pwds_left } = state;
    dev.build_input_stream(
        &conf.into(),
        move |data: &[T], _| {
            let mut vec = Vec::<u8>::new();
            for samp in data {
                vec.push(samp.aggregate_bits());
            }
            for pwd in gen.push(&vec) {
                match String::from_utf8(pwd) {
                    Ok(pwd) => println!("{}", pwd),
                    Err(_) => println!("(invalid UTF string)"),
                }
                num_pwds_left -= 1;
                if num_pwds_left == 0 {
                    match mutex.get_mut() {
                        Ok(tx) => if let Err(e) = tx.send(Ok(())) {
                            eprintln!("{}", e);
                        },
                        Err(_) => (),
                    }
                    return;
                }
            }
        },
        |e| eprintln!("{}", e),
    ).map_err(|e| e.to_string())
}
