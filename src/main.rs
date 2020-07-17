use core::result;
use std::env;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;

use cpal::SampleFormat::*;
use cpal::traits::*;

use crate::generator::Generator;
use crate::params::Params;
use crate::sample::stream;

mod generator;
mod bit_vec;
mod params;
mod sample;

type Result<T> = result::Result<T, String>;

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
        gen: Generator::new(params.cset, params.blk_len, params.pwd_len),
        mutex: Mutex::new(tx),
        num_pwds_left: params.num_pwds
    };
    let host = cpal::default_host();
    let dev = match host.default_input_device() {
        Some(dev) => Ok(dev),
        None => Err("No audio input device available"),
    }?;
    let conf = dev.default_input_config().map_err(|e| e.to_string())?;
    let stream = match conf.sample_format() {
        I16 => stream::<i16>(dev, conf.into(), state),
        U16 => stream::<u16>(dev, conf.into(), state),
        F32 => stream::<f32>(dev, conf.into(), state),
    }.map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    rx.recv().map_err(|e| e.to_string())??;
    drop(stream);
    Ok(())
}
