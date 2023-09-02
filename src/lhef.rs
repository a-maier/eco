use std::{io::BufWriter, fs::File};

use anyhow::Result;
use avery::Event;
use lhef::HEPRUP;

use crate::writer::WriteEv;

pub(crate) struct Writer (
    lhef::Writer<BufWriter<File>>
);
impl Writer {
    pub(crate) fn new<P: AsRef<std::path::Path>>(outfile: P) -> Result<Self> {
        let file = File::create(outfile)?;
        let writer = lhef::Writer::new(BufWriter::new(file), "1.0")?;
        Ok(Self(writer))
    }

    fn write_header(&mut self, heprup: HEPRUP) -> Result<()> {
        self.0.heprup(&heprup)?;
        Ok(())
    }
}

impl WriteEv for Writer {
    fn write(&mut self, event: Event) -> Result<()> {
        use lhef::writer::WriterState::ExpectingHeaderOrInit;

        let hepeup = if self.0.state() == ExpectingHeaderOrInit {
            let (heprup, ev) = event.into();
            self.write_header(heprup)?;
            ev
        } else {
            event.into()
        };
        self.0.hepeup(&hepeup)?;
        Ok(())
    }
}
