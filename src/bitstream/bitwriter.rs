use std::io::Write;

use crate::bitstream::ToBits;
use crate::error::*;

// TODO: This is really just a skeleton so far...
pub struct BitWriter<W> {
    writer: W,
}

impl<W> BitWriter<W>
where W: Write,
{
    pub fn new(writer: W) -> Self { Self { writer } }

    pub fn write<T>(&mut self, value: T) -> Result<()>
    where
        T: ToBits<W>,
    {
        value.to_bits(self)
    }
}
