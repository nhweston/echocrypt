use std::ops::Receiver;
use std::sync::mpsc::{channel, Sender};

use soundio::{ChannelLayout, ChannelLayoutId, Device, Format, InStreamReader};

use crate::generator::Generator;
use crate::Result;

pub fn run(
    dev: Device,
    cset: Vec<u8>,
    blk_len: usize,
    pwd_len: usize,
    num_pwds: usize,
) -> Result<()> {
    let mut gen = Generator::new(cset, blk_len, pwd_len);
    let (tx, rx) = channel();
    let mut num_pwds_left = num_pwds;
    let callback = |stream| {
        let num_frames = stream.frame_count_max();
        if let Err(e) = stream.begin_read(num_frames) {
            tx.send(Err(e.to_string()));
            return;
        }
        let mut bytes = Vec::with_capacity(num_frames);
        for frame in 0..num_frames {
            data.push(stream.sample::<u8>(0, frame));
        }
        for pwd in gen.push(bytes) {
            match String::from_utf8(pwd) {
                Ok(pwd) => println!("{}", pwd),
                Err(_) => println!("(invalid UTF string)"),
            }
            num_pwds_left -= 1;
            if num_pwds_left == 0 {
                tx.send(Ok(()));
                return;
            }
        }
    };
    let mut stream = dev.open_instream(
        44100,
        Format::U16LE,
        ChannelLayout::get_builtin(ChannelLayoutId::Mono),
        0.1,
        callback,
        None::<fn()>,
        None::<fn(soundio::Error)>,
    )?;
    stream.start()?;
    rx.recv()?;
    Ok(())
}
