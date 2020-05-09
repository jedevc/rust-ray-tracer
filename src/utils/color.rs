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
    let r = clamp(color.x / (samples as f64), 0.0, 0.999);
    let g = clamp(color.y / (samples as f64), 0.0, 0.999);
    let b = clamp(color.z / (samples as f64), 0.0, 0.999);

    writeln!(
        writer, "{} {} {}",
        (r * 256.0) as u8,
        (g * 256.0) as u8,
        (b * 256.0) as u8
    )
}
