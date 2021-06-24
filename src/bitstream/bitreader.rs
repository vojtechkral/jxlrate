use std::io::Read;

use crate::bitstream::FromBits;
use crate::error::*;

/// Pull individual bits or bit ranges up to 56 bits per call
/// out of any `io::Read` type.
pub struct BitReader<R> {
    reader: R,
    bits: u64,
    num_bits: u32,
    bits_read: u64,
}

impl<R> BitReader<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            bits: 0,
            num_bits: 0,
            bits_read: 0,
        }
    }

    pub fn peek_bits(&mut self, bits: u32) -> Result<u64> {
        assert!(bits <= 56);

        let mut needed = bits.saturating_sub(self.num_bits);
        while needed > 0 {
            let mut bytes = (&mut self.reader).bytes();
            let byte = bytes.next().ok_or(Error::UnexpectedEof)??;

            self.bits |= (byte as u64) << (self.num_bits as u64);
            self.num_bits += 8;
            needed = needed.saturating_sub(8);
        }

        let res = self.bits & ((1 << bits as u64) - 1);
        Ok(res)
    }

    pub fn read_bits(&mut self, bits: u32) -> Result<u64> {
        let res = self.peek_bits(bits)?;
        self.num_bits -= bits;
        self.bits >>= bits;
        self.bits_read += bits as u64;
        Ok(res)
    }

    pub fn read<T>(&mut self) -> Result<T>
    where
        T: FromBits<R>,
    {
        T::from_bits(self)
    }

    pub fn into_inner(self) -> R {
        self.reader
    }

    pub fn bits_read(&self) -> u64 {
        self.bits_read
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_reader() {
        let data = b"Hello, World!";

        let mut reader = BitReader::new(&data[..]);
        assert_eq!(reader.peek_bits(3).unwrap(), 0);
        assert_eq!(reader.read_bits(4).unwrap(), 8);
        assert_eq!(reader.peek_bits(16).unwrap(), 0b1100_0110_0101_0100);
        assert_eq!(reader.read_bits(7).unwrap(), 0b101_0100);
        assert_eq!(reader.read_bits(9).unwrap(), 0b1100_0110_0);
    }
}
