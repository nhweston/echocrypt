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

pub struct State {
    gen: Generator,
    mutex: Mutex<Sender<Result<()>>>,
    num_pwds_left: usize,
}

fn main() {
    if let Err(msg) = run() {
        eprintln!("{}", msg);
    }
}

fn run() -> Result<()> {
    let params = Params::new(env::args().collect())?;
    let (tx, rx) = channel::<Result<()>>();
    let state = State {
        gen: Generator::new(params.cset, params.pwd_len),
        mutex: Mutex::new(tx),
        num_pwds_left: params.num_pwds
    };
    let host = cpal::default_host();
    let dev = host.default_input_device().ok_or(anyhow!("No device available"))?;
    let conf = dev.default_input_config()?;
    let stream = match conf.sample_format() {
        I16 => stream::<i16>(dev, conf.into(), state),
        U16 => stream::<u16>(dev, conf.into(), state),
        F32 => stream::<f32>(dev, conf.into(), state),
    }?;
    stream.play()?;
    rx.recv()??;
    drop(stream);
    Ok(())
}

pub fn stream<T: Sample>(
    dev: Device,
    conf: StreamConfig,
    state: State,
) -> Result<Stream> {
    let State { mut gen, mut mutex, mut num_pwds_left } = state;
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
    )?;
    Ok(stream)
}
