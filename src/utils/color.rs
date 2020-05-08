use std::io;

use super::vec;

type Color = vec::Vec;

pub fn write_color(writer: &mut impl io::Write, color: Color) -> io::Result<()> {
    writeln!(writer, "{} {} {}",
        (color.x * 255.999) as u8,
        (color.y * 255.999) as u8,
        (color.z * 255.999) as u8)
}