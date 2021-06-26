use std::io::Read;
use std::marker::PhantomData;
use std::cmp::{Ordering, PartialEq, PartialOrd};

use typenum::{Unsigned, U2};

use crate::bitstream::{FromBits, BitReader};
use crate::error::*;

impl<R> FromBits<R> for bool
where
    R: Read,
{
    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> {
        reader.read_bits(1).map(|b| b == 1)
    }

    fn read_bits(&mut self, reader: &mut BitReader<R>) -> Result<()> {
        *self = Self::from_bits(reader)?;
        Ok(())
    }
}

/// Static constant number (known ahead of time),
/// takes zero bits in the bitstream.
/// Mostly useful with `PrefixU32`.
#[derive(Clone, Copy, Default, Debug)]
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

    fn read_bits(&mut self, reader: &mut BitReader<R>) -> Result<()> {
        Ok(())
    }
}

/// N-bit direct-coded number (max 32 bits, LE).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bits<N>(pub u32, PhantomData<N>);

impl<N> From<u32> for Bits<N> {
    fn from(u: u32) -> Self {
        Self(u, PhantomData)
    }
}

impl<N> From<Bits<N>> for u32 {
    fn from(this: Bits<N>) -> Self {
        this.0
    }
}

impl<N> PartialEq<u32> for Bits<N> {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl<N> PartialOrd<u32> for Bits<N> {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
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

    fn read_bits(&mut self, reader: &mut BitReader<R>) -> Result<()> {
        *self = Self::from_bits(reader)?;
        Ok(())
    }
}

/// N-bit number encoded with an offest subtracted first,
/// then coding directly (LE).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BitsOffset<N, O>(pub u32, PhantomData<(N, O)>);

impl<N, O> From<u32> for BitsOffset<N, O> {
    fn from(u: u32) -> Self {
        Self(u, PhantomData)
    }
}

impl<N, O> From<BitsOffset<N, O>> for u32 {
    fn from(this: BitsOffset<N, O>) -> Self {
        this.0
    }
}

impl<N, O> PartialEq<u32> for BitsOffset<N, O> {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl<N, O> PartialOrd<u32> for BitsOffset<N, O> {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
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

    fn read_bits(&mut self, reader: &mut BitReader<R>) -> Result<()> {
        *self = Self::from_bits(reader)?;
        Ok(())
    }
}

/// TODO: Explain this stuff
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PrefixU32<D0, D1, D2, D3>(pub u32, PhantomData<fn() -> (D0, D1, D2, D3)>);

impl<D0, D1, D2, D3> From<u32> for PrefixU32<D0, D1, D2, D3> {
    fn from(u: u32) -> Self {
        Self(u, PhantomData)
    }
}

impl<D0, D1, D2, D3> From<PrefixU32<D0, D1, D2, D3>> for u32 {
    fn from(this: PrefixU32<D0, D1, D2, D3>) -> Self {
        this.0
    }
}

impl<D0, D1, D2, D3> PartialEq<u32> for PrefixU32<D0, D1, D2, D3> {
        fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl<D0, D1, D2, D3> PartialOrd<u32> for PrefixU32<D0, D1, D2, D3> {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<R, D0, D1, D2, D3> FromBits<R> for PrefixU32<D0, D1, D2, D3>
where
    R: Read,
    D0: FromBits<R> + Into<u32> + Default,
    D1: FromBits<R> + Into<u32> + Default,
    D2: FromBits<R> + Into<u32> + Default,
    D3: FromBits<R> + Into<u32> + Default,
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

    fn read_bits(&mut self, reader: &mut BitReader<R>) -> Result<()> {
        *self = Self::from_bits(reader)?;
        Ok(())
    }
}
