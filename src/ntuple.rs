use std::path::Path;

use anyhow::{anyhow, Result};
use avery::Event;

use crate::writer::WriteEv;

pub(crate) struct Reader (
    ntuple::Reader
);

impl Reader {
    pub(crate) fn new<P: AsRef<Path>>(input: P) -> Result<Self> {
        let reader = ntuple::Reader::new(input).ok_or_else(
            || anyhow!("Failed to read from ROOT file")
        )?;
        Ok(Self(reader))
    }
}

impl Iterator for Reader {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn last(self) -> Option<Self::Item> {
        self.0.last().map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }
}

pub(crate) struct Writer (
    ntuple::Writer
);

impl Writer {
    pub(crate) fn new<P: AsRef<std::path::Path>>(outfile: P) -> Result<Self> {
        let writer = ntuple::Writer::new(outfile, "").ok_or_else(
            || anyhow!("Failed to write to ROOT file")
        )?;
        Ok(Self(writer))
    }
}

impl WriteEv for Writer {
    fn write(&mut self, event: Event) -> Result<()> {
        self.0.write(&event.into())?;
        Ok(())
    }
}
