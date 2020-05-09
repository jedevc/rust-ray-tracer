use std::io;

use super::vec;
use std::cmp::PartialOrd;

pub type Color = vec::Vec;

fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn write_color(writer: &mut impl io::Write, color: Color, samples: u32) -> io::Result<()> {
    let r = color.x / (samples as f64);
    let g = color.y / (samples as f64);
    let b = color.z / (samples as f64);

    // sqrt to gamma correct for gamma=2.0
    let r = clamp(r.sqrt(), 0.0, 0.999);
    let g = clamp(g.sqrt(), 0.0, 0.999);
    let b = clamp(b.sqrt(), 0.0, 0.999);

    writeln!(
        writer, "{} {} {}",
        (r * 256.0) as u8,
        (g * 256.0) as u8,
        (b * 256.0) as u8
    )
}
