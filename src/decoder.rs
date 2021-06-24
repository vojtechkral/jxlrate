use std::io::Read;

use typenum::consts::*;

use crate::bitstream::{BitReader, Bits, BitsOffset, FromBits, PrefixU32, Rational};
use crate::error::*;

static MAGIC_SINGLE: u64 = 0x0aff;

type SizeSmall = Bits<U5>;
type SizeLarge =
    PrefixU32<BitsOffset<U9, U1>, BitsOffset<U13, U1>, BitsOffset<U18, U1>, BitsOffset<U30, U1>>;

enum Size {
    Small(SizeSmall),
    Large(SizeLarge),
}

impl Size {
    fn read<R>(reader: &mut BitReader<R>, small: bool) -> Result<Self>
    where
        R: Read,
    {
        Ok(if small {
            Self::Small(reader.read()?)
        } else {
            Self::Large(reader.read()?)
        })
    }
}

impl From<Size> for u32 {
    fn from(this: Size) -> Self {
        match this {
            Size::Small(s) => 8 * (s.0 + 1),
            Size::Large(s) => s.0,
        }
    }
}

#[derive(Debug)]
pub struct Header {
    width: u32,
    height: u32,
}

impl Header {
    const RATIOS: [Rational; 7] = [
        Rational(1, 1),
        Rational(12, 10),
        Rational(4, 3),
        Rational(3, 2),
        Rational(16, 9),
        Rational(5, 4),
        Rational(2, 1),
    ];
}

impl<R> FromBits<R> for Header
where
    R: Read,
{
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> {
        let small: bool = reader.read()?;
        let height = Size::read(reader, small)?.into();
        let ratio: Bits<U3> = reader.read()?;

        let width = if ratio.0 != 0 {
            let ratio = Self::RATIOS[ratio.0 as usize - 1];
            ratio * height
        } else {
            Size::read(reader, small)?.into()
        };

        dbg!(reader.bits_read());

        Ok(Self { width, height })
    }
}

pub struct Decoder<R> {
    reader: BitReader<R>,
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader: BitReader::new(reader),
        }
    }

    pub fn read_header(mut self) -> Result<(Header, R)> {
        let signature = self.reader.read_bits(16)?;
        if signature != MAGIC_SINGLE {
            return Err(Error::BadSignature);
        }

        let header = Header::from_bits(&mut self.reader)?;
        Ok((header, self.reader.into_inner()))
    }
}
