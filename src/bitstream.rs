mod bitreader;
mod bitwriter;
mod types;

use crate::error::*;

pub use bitreader::BitReader;
pub use bitwriter::BitWriter;
pub use types::*;

/// Methods for decoding a type from the bitstream.
/// Implementations for basic types are defined in `types`.
pub trait FromBits<R>: Sized {
    fn read_bits(&mut self, reader: &mut BitReader<R>) -> Result<()>;

    fn from_bits(reader: &mut BitReader<R>) -> Result<Self> where Self: Default {
        let mut this: Self = Default::default();
        this.read_bits(reader)?;
        Ok(this)
    }
}

/// Methods for encoding a type to the bitstream.
/// Implementations for basic types are defined in `types`.
pub trait ToBits<W>: Sized {
    fn to_bits(&self, writer: &mut BitWriter<W>) -> Result<()>;
}

macro_rules! bitstream {
    (
        $ty:ty as $self:ident
        fields {
            $(
                $([if $cond:expr])? $field:ident : $fty:ty = $default:expr ,
            )+
        }
        read $read:block
        write $write:block
    ) => {
        impl<R> $crate::bitstream::FromBits<R> for $ty
        where
            R: std::io::Read,
        {
            fn read_bits(&mut self, reader: &mut $crate::bitstream::BitReader<R>) -> $crate::error::Result<()> {
                let $self = self;

                $(
                    bitstream!(@field_read reader, $field, $fty, $default $(, $cond)?);
                )+

                $read
            }
        }

        impl<W> $crate::bitstream::ToBits<W> for $ty
        where
            W: std::io::Write,
        {
            #[allow(unreachable_code)]
            fn to_bits(&self, writer: &mut $crate::bitstream::BitWriter<W>) -> $crate::error::Result<()> {
                $(
                    bitstream!(@field_write_decl $field, $fty, $default $(, $cond)?);
                )+

                #[allow(unused)]
                let $self = self;
                $write

                $(
                    bitstream!(@field_write writer, $field, $fty, $default $(, $cond)?);
                )+

                Ok(())
            }
        }
    };

    (@field_read $reader:ident, $field:ident, $fty:ty, $default:expr) => {
        let $field: $fty = $reader.read()?;
    };

    (@field_read $reader:ident, $field:ident, $fty:ty, $default:expr, $cond:expr) => {
        let $field: $fty = if $cond {
            $reader.read()?
        } else {
            $default.into()
        };
    };

    (@field_write_decl $field:ident, $fty:ty, $default:expr) => {
        #[allow(unused)]
        #[allow(unused_mut)]
        let mut $field: $fty = $default.into();
    };

    (@field_write_decl $field:ident, $fty:ty, $default:expr, $cond:expr) => {
        #[allow(unused)]
        #[allow(unused_mut)]
        let mut $field: $fty = $default.into();
    };

    (@field_write $writer:ident, $field:ident, $fty:ty, $default:expr) => {
        // $writer.write($field)?;
    };

    (@field_write $writer:ident, $field:ident, $fty:ty, $default:expr, $cond:expr) => {
        if $cond {
            // $writer.write($field)?;
        }
    };
}
