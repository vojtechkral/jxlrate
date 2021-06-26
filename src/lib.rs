#[macro_use] pub(crate) mod bitstream;
pub(crate) mod decoder;
pub(crate) mod error;
pub(crate) mod utils;

pub use decoder::Decoder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
