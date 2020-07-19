use std::env;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;

use anyhow::*;
use cpal::{Device, Stream, StreamConfig};
use cpal::SampleFormat::*;
use cpal::traits::*;

use crate::generator::Generator;
use crate::params::Params;
use crate::sample::Sample;

mod bit_vec;
mod byte_set;
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
    let dev = host.default_input_device().ok_or(anyhow!("No device available"))?;
    let (tx, rx) = channel::<Result<()>>();
    let (conf, samp_fmt) = {
        let conf = dev.default_input_config()?;
        (conf.config(), conf.sample_format())
    };
    let stream = match samp_fmt {
        I16 => stream::<i16>(dev, conf, params, tx),
        U16 => stream::<u16>(dev, conf, params, tx),
        F32 => stream::<f32>(dev, conf, params, tx),
    }?;
    stream.play()?;
    rx.recv()??;
    drop(stream);
    Ok(())
}

pub fn stream<T: Sample>(
    dev: Device,
    conf: StreamConfig,
    params: Params,
    tx: Sender<Result<()>>,
) -> Result<Stream> {
    let Params { cset, pwd_len, num_pwds } = params;
    let mut gen = Generator::new(cset, pwd_len);
    let mut mutex = Mutex::new(tx);
    let mut num_pwds_left = num_pwds;
    let stream = dev.build_input_stream(
        &conf.into(),
        move |data: &[T], _| {
            let mut vec = Vec::new();
            for samp in data {
                vec.push(samp.aggregate_sample());
            }
            for pwd in gen.push(&vec) {
                match String::from_utf8(pwd) {
                    Ok(pwd) => println!("{}", pwd),
                    Err(_) => println!("(invalid UTF string)"),
                }
                num_pwds_left -= 1;
                if num_pwds_left == 0 {
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
    Ok(stream)
}
