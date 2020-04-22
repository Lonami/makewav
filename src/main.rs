// Useful resources:
//
// * http://soundfile.sapp.org/doc/WaveFormat/
// * http://www.topherlee.com/software/pcm-tut-wavformat.html
use std::io::{self, BufWriter, Write};
use std::time::Duration;

const DURATION: Duration = Duration::from_secs(2);
const SAMPLE_RATE: u32 = 44100;
const BIT_SIZE: u16 = 8;
const NUM_CHANNELS: u16 = 1;

const HEADER_LEN: u32 = 44;

/// The function you want to modify to generate more interesting sounds.
fn sample(t: f32) -> u8 {
    (((1000.0 * t).sin() + 1.0) * 127.0) as u8
}

fn main() -> io::Result<()> {
    let sample_count = ((SAMPLE_RATE as u128 * DURATION.as_nanos()) / 1_000_000_000) as u32;

    let out = io::stdout();
    let mut writer = BufWriter::new(out.lock());

    // RIFF header
    writer.write_all(b"RIFF")?;
    writer.write_all(&(HEADER_LEN + sample_count - 8).to_le_bytes())?;
    writer.write_all(b"WAVE")?;

    // fmt chunk
    writer.write_all(b"fmt ")?;
    writer.write_all(&16u32.to_le_bytes())?;
    writer.write_all(&1u16.to_le_bytes())?;
    writer.write_all(&NUM_CHANNELS.to_le_bytes())?;
    writer.write_all(&SAMPLE_RATE.to_le_bytes())?;
    writer.write_all(&((SAMPLE_RATE * BIT_SIZE as u32 * NUM_CHANNELS as u32) / 8).to_le_bytes())?;
    writer.write_all(&((BIT_SIZE * NUM_CHANNELS) / 8).to_le_bytes())?;
    writer.write_all(&BIT_SIZE.to_le_bytes())?;

    // data chunk
    writer.write_all(b"data")?;
    writer.write_all(&sample_count.to_le_bytes())?;

    let scale = 1.0 / SAMPLE_RATE as f32;
    for t in (0..sample_count).map(|s| s as f32 * scale) {
        writer.write_all(&[sample(t)])?;
    }

    Ok(())
}
