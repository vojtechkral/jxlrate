use std::fs::File;
use std::path::PathBuf;

use lazy_static::lazy_static;

use jxlrate::Decoder;

lazy_static! {
    pub static ref IMGS: PathBuf = {
        [env!("CARGO_MANIFEST_DIR"), "tests", "imgs"]
            .as_ref()
            .into_iter()
            .collect()
    };
}

#[test]
fn read_header_large() {
    let file = File::open(IMGS.join("resf.jxl")).unwrap();
    let decoder = Decoder::new(file);
    let header = decoder.read_header().unwrap().0;
    dbg!(header);
}

#[test]
fn read_header_ratio() {
    let file = File::open(IMGS.join("ratio.jxl")).unwrap();
    let decoder = Decoder::new(file);
    let header = decoder.read_header().unwrap().0;
    dbg!(header);
}
