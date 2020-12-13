use std::env;
use std::sync::mpsc::channel;
use std::sync::Mutex;

use anyhow::*;
use cpal::{Device, StreamConfig};
use cpal::SampleFormat::*;
use cpal::traits::*;

use crate::generator::Generator;
use crate::params::Params;
use crate::sample::Sample;

mod bit_vec;
mod byte_set;
mod charset;
mod generator;
mod params;
mod sample;

fn main() {
    if let Err(msg) = run() {
        eprintln!("{}", msg);
    }
}

fn run() -> Result<()> {
    let params = Params::new(env::args().collect())?;
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or(anyhow!("No device available"))?;
    let (conf, samp_fmt) = {
        let conf = device.default_input_config()?;
        (conf.config(), conf.sample_format())
    };
    match samp_fmt {
        I16 => start::<i16>(device, conf, params),
        U16 => start::<u16>(device, conf, params),
        F32 => start::<f32>(device, conf, params),
    }
}

pub fn start<T: Sample>(
    dev: Device,
    conf: StreamConfig,
    params: Params,
) -> Result<()> {
    let (tx, rx) = channel::<Result<()>>();
    let Params { charset, pwd_len, num_pwds } = params;
    let mut gen = Generator::new(charset, pwd_len);
    let mut mutex = Mutex::new(tx);
    let mut num_pwds_remaining = num_pwds;
    let border = "─".repeat(pwd_len);
    println!("┌{}┐", border);
    let stream = dev.build_input_stream(
        &conf.into(),
        move |data: &[T], _| {
            let mut vec = Vec::new();
            for &samp in data {
                vec.push(samp.aggregate_sample());
            }
            for pwd in gen.push(&vec) {
                match String::from_utf8(pwd) {
                    Ok(pwd) => println!("│{}│", pwd),
                    Err(_) => println!("(invalid UTF string)"),
                }
                num_pwds_remaining -= 1;
                if num_pwds_remaining == 0 {
                    println!("└{}┘", border);
                    match mutex.get_mut() {
                        Ok(tx) => {
                            if let Err(e) = tx.send(Ok(())) {
                                eprintln!("{}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("{}", e);
                        },
                    }
                    return;
                }
            }
        },
        |e| eprintln!("{}", e),
    )?;
    stream.play()?;
    rx.recv()??;
    Ok(())
}
