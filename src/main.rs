use anyhow::{Result, anyhow};
use fdk_aac::dec::{Decoder, Transport};
use libpulse_binding::{
    sample::{Format, Spec},
    stream::Direction,
};
use libpulse_simple_binding::Simple;
use std::{
    fs,
    io::{Cursor, Write},
    slice,
};

const PCM_FRAME_SIZE: usize = 4 * 1024;

fn main() -> Result<()> {
    let input = fs::read("/home/dyn/Downloads/1.aac")?;

    let mut output = Cursor::new(vec![]);

    let mut decoder = Decoder::new(Transport::Adts);
    decoder.fill(&input).map_err(|e| anyhow!(e))?;
    loop {
        let mut buf = vec![0; PCM_FRAME_SIZE];
        if let Err(e) = decoder.decode_frame(&mut buf) {
            dbg!(e);
            break;
        }

        let size = decoder.decoded_frame_size();

        let s = unsafe { slice::from_raw_parts(buf.as_ptr() as *const u8, size * 2) };

        output.write_all(s)?;
    }

    let spec = Spec {
        format: Format::S16le,
        channels: 2,
        rate: 48000,
    };

    let ps = Simple::new(
        None,
        "rajiko-cli",
        Direction::Playback,
        None,
        "play",
        &spec,
        None,
        None,
    )?;

    ps.write(output.get_ref())?;
    ps.drain()?;
    Ok(())
}
