use std::{io::{BufRead, BufWriter}, fs::File};

use anyhow::Result;
use avery::Event;

use crate::writer::WriteEv;

pub(crate) struct Reader<T> (
    hepmc2::Reader<T>
);

impl<T: BufRead> Reader<T> {
    pub(crate) fn new(input: T) -> Result<Self> {
        let reader = hepmc2::Reader::new(input);
        Ok(Self(reader))
    }
}

impl<T: BufRead> Iterator for Reader<T> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }
}

pub(crate) struct Writer (
    hepmc2::Writer<BufWriter<File>>
);
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
