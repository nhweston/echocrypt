use cpal::{Device, Stream, StreamConfig};
use cpal::traits::DeviceTrait;

use crate::{Result, State};

pub trait Sample where Self: Sized + cpal::Sample {
    fn convert_to_u8(self) -> u8;
}

impl Sample for i16 {
    fn convert_to_u8(self) -> u8 {
        self as u8
    }
}

impl Sample for u16 {
    fn convert_to_u8(self) -> u8 {
        self as u8
    }
}

impl Sample for f32 {
    fn convert_to_u8(self) -> u8 {
        ((self + 1.0) * (std::u16::MAX as f32)) as u16 as u8
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
                vec.push(samp.convert_to_u8());
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
