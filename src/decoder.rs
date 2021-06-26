use std::io::Read;

use typenum::consts::*;

use crate::bitstream::{BitReader, Bits, BitsOffset, FromBits, PrefixU32};
use crate::error::*;
use crate::utils::Rational;

static MAGIC_SINGLE: u64 = 0x0aff;

type SizeSmall = Bits<U5>;
type SizeLarge =
    PrefixU32<BitsOffset<U9, U1>, BitsOffset<U13, U1>, BitsOffset<U18, U1>, BitsOffset<U30, U1>>;

#[derive(Debug)]
pub struct SizeHeader {
    pub width: u32,
    pub height: u32,
}

impl SizeHeader {
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

bitstream!(SizeHeader where
    fields {
        small: bool = false,
        [if small] h_small: SizeSmall = 0,
        [if !small] h_large: SizeLarge = 0,
        ratio: Bits<U3> = 0,
        [if ratio == 0 && small] w_small: SizeSmall = 0,
        [if ratio == 0 && !small] w_large: SizeLarge = 0,
    }
    read {
        let height = if small { h_small.0 } else { h_large.0 };
        let width = if ratio > 0 {
            SizeHeader::RATIOS[ratio.0 as usize - 1] * height
        } else if small {
            w_small.0
        } else {
            w_large.0
        };

        Ok(SizeHeader { width, height })
    }
    write this {
        todo!()
    }
);

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

    pub fn read_header(mut self) -> Result<(SizeHeader, R)> {
        let signature = self.reader.read_bits(16)?;
        if signature != MAGIC_SINGLE {
            return Err(Error::BadSignature);
        }

        let header = SizeHeader::from_bits(&mut self.reader)?;
        Ok((header, self.reader.into_inner()))
    }
}
