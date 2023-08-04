use std::{io::{BufRead, BufWriter}, fs::File};

use anyhow::Result;
use avery::Event;
use lhef::HEPRUP;

use crate::writer::WriteEv;

pub(crate) struct Reader<T> (
    lhef::Reader<T>
);

impl<T: BufRead> Reader<T> {
    pub(crate) fn new(input: T) -> Result<Self> {
        let reader = lhef::Reader::new(input)?;
        Ok(Self(reader))
    }
}

impl<T: BufRead> Iterator for Reader<T> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        let ev = self.0.hepeup();
        match ev {
            Ok(Some(ev)) => Some(Ok((self.0.heprup().to_owned(), ev).into())),
            Ok(None) => None,
            Err(err) => Some(Err(err.into())),
        }
    }
}

pub(crate) struct Writer (
    lhef::Writer<BufWriter<File>>
);
impl Writer {
    pub(crate) fn new<P: AsRef<std::path::Path>>(outfile: P) -> Result<Self> {
        let file = File::open(outfile)?;
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
