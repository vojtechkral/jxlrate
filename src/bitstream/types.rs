use std::io::Read;
use std::marker::PhantomData;
use std::ops;

use typenum::{Unsigned, U2};

use crate::bitstream::BitReader;
use crate::error::*;

/// Types that can be decoded from the bitstream
pub trait FromBits<R>: Sized {
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self>;
}

impl<R> FromBits<R> for bool
where
    R: Read,
{
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> {
        reader.read_bits(1).map(|b| b == 1)
    }
}

/// Static constant number (known ahead of time),
/// takes zero bits in the bitstream.
/// Mostly useful with `PrefixU32`.
pub struct Const<X>(PhantomData<X>);

impl<R, X> FromBits<R> for Const<X>
where
    R: Read,
    X: Unsigned,
{
    fn from_bits(_: &mut BitReader<R>) -> Result<Self> {
        // Const value, ie. no reading from the bitstream needed
        Ok(Self(PhantomData))
    }
}

/// N-bit direct-coded number (max 32 bits, LE).
pub struct Bits<N>(pub u32, PhantomData<N>);

impl<N> From<Bits<N>> for u32 {
    fn from(this: Bits<N>) -> Self {
        this.0
    }
}

impl<R, N> FromBits<R> for Bits<N>
where
    R: Read,
    N: Unsigned,
{
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> {
        let x = reader.read_bits(N::to_u32())?;
        Ok(Self(x as u32, PhantomData))
    }
}

/// N-bit number encoded with an offest subtracted first,
/// then coding directly (LE).
pub struct BitsOffset<N, O>(pub u32, PhantomData<(N, O)>);

impl<N, O> From<BitsOffset<N, O>> for u32 {
    fn from(this: BitsOffset<N, O>) -> Self {
        this.0
    }
}

impl<R, N, O> FromBits<R> for BitsOffset<N, O>
where
    R: Read,
    N: Unsigned,
    O: Unsigned,
{
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> {
        let x = reader.read_bits(N::to_u32())?;
        let x = x as u32 + O::to_u32();
        Ok(Self(x, PhantomData))
    }
}

/// TODO: Explain this stuff
pub struct PrefixU32<D0, D1, D2, D3>(pub u32, PhantomData<fn() -> (D0, D1, D2, D3)>);

impl<D0, D1, D2, D3> From<PrefixU32<D0, D1, D2, D3>> for u32 {
    fn from(this: PrefixU32<D0, D1, D2, D3>) -> Self {
        this.0
    }
}

impl<R, D0, D1, D2, D3> FromBits<R> for PrefixU32<D0, D1, D2, D3>
where
    R: Read,
    D0: FromBits<R> + Into<u32>,
    D1: FromBits<R> + Into<u32>,
    D2: FromBits<R> + Into<u32>,
    D3: FromBits<R> + Into<u32>,
{
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> {
        let selector = Bits::<U2>::from_bits(reader)?.0;

        let x = match selector {
            0 => reader.read::<D0>()?.into(),
            1 => reader.read::<D1>()?.into(),
            2 => reader.read::<D2>()?.into(),
            3 => reader.read::<D3>()?.into(),
            _ => unreachable!(),
        };

        Ok(Self(x, PhantomData))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rational(pub u32, pub u32);

impl ops::Mul<u32> for Rational {
    type Output = u32;

    fn mul(self, rhs: u32) -> Self::Output {
        (rhs as u64 * self.0 as u64) as u32 / self.1
    }
}
