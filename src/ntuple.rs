use anyhow::{anyhow, Result};
use avery::Event;

use crate::writer::WriteEv;

pub(crate) struct Writer(ntuple::Writer);

impl Writer {
    pub(crate) fn new<P: AsRef<std::path::Path>>(outfile: P) -> Result<Self> {
        let writer = ntuple::Writer::new(outfile, "")
            .ok_or_else(|| anyhow!("Failed to write to ROOT file"))?;
        Ok(Self(writer))
    }
}

impl WriteEv for Writer {
    fn write(&mut self, event: Event) -> Result<()> {
        self.0.write(&event.into())?;
        Ok(())
    }
}
