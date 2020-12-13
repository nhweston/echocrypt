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
    let (config, sample_format) = {
        let config = device.default_input_config()?;
        (config.config(), config.sample_format())
    };
    match sample_format {
        I16 => start::<i16>(device, config, params),
        U16 => start::<u16>(device, config, params),
        F32 => start::<f32>(device, config, params),
    }
}

pub fn start<T: Sample>(
    device: Device,
    config: StreamConfig,
    params: Params,
) -> Result<()> {
    let (tx, rx) = channel::<Result<()>>();
    let Params { charset, pwd_len, num_pwds } = params;
    let mut generator = Generator::new(charset, pwd_len);
    let mut mutex = Mutex::new(tx);
    let mut num_pwds_remaining = num_pwds;
    let border = "─".repeat(pwd_len);
    println!("┌{}┐", border);
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[T], _| match mutex.get_mut() {
            Ok(tx) => {
                let mut vec = Vec::new();
                for &sample in data {
                    vec.push(sample.aggregate_sample());
                }
                for pwd in generator.push(&vec) {
                    match String::from_utf8(pwd) {
                        Ok(pwd) => println!("│{}│", pwd),
                        Err(_) => println!("(invalid UTF string)"),
                    }
                    num_pwds_remaining -= 1;
                    if num_pwds_remaining == 0 {
                        if let Err(e) = tx.send(Ok(())) {
                            eprintln!("{}", e);
                        }
                        println!("└{}┘", border);
                        return;
                    }
                }
            },
            Err(e) => {
                eprintln!("{}", e);
            },
        },
        |e| eprintln!("{}", e),
    )?;
    stream.play()?;
    rx.recv()??;
    Ok(())
}
