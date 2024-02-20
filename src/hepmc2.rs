use std::{fs::File, io::BufWriter};

use anyhow::Result;
use avery::Event;

use crate::writer::WriteEv;

pub(crate) struct Writer(hepmc2::Writer<BufWriter<File>>);
impl Writer {
    pub(crate) fn new<P: AsRef<std::path::Path>>(outfile: P) -> Result<Self> {
        let file = File::create(outfile)?;
        let writer = hepmc2::Writer::new(BufWriter::new(file))?;
        Ok(Self(writer))
    }
}

impl WriteEv for Writer {
    fn write(&mut self, event: Event) -> Result<()> {
        self.0.write(&event.into())?;
        Ok(())
    }
}
