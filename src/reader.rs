use std::{io::BufReader, fs::File};

use anyhow::{anyhow, Result};
use audec::auto_decompress;
use avery::Event;

pub(crate) struct Reader (
    Box<dyn Iterator<Item = Result<Event>>>
);

impl Reader {
    pub(crate) fn new(infile: &str) -> Result<Self> {
        let file = File::open(infile)?;
        let mut input = auto_decompress(BufReader::new(file));
        let buf = input.fill_buf()?;
        #[cfg(feature = "ntuple")]
        if buf.starts_with(b"root") {
            return Ok(Self(Box::new(crate::ntuple::Reader::new(infile)?)));
        }
        // TODO: trim_ascii_start as soon as it's stable
        let buf = trim_ascii_start(buf);
        #[cfg(feature = "lhef")]
        if buf.starts_with(b"<LesHouchesEvents") {
            return Ok(Self(Box::new(crate::lhef::Reader::new(input)?)));
        }
        #[cfg(feature = "hepmc2")]
        if buf.starts_with(b"HepMC") {
            return Ok(Self(Box::new(crate::hepmc2::Reader::new(input)?)));
        }
        Err(anyhow!("Failed to determine event file format"))
    }
}

impl Iterator for Reader {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn trim_ascii_start(buf: &[u8]) -> &[u8] {
    if let Some(pos) = buf.iter().position(|b| !b.is_ascii_whitespace()) {
        &buf[pos..]
    } else {
        &[]
    }
}
