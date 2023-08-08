use std::path::Path;

use anyhow::{Result, bail};
use avery::Event;

use crate::format::Format;

pub(crate) struct Writer (
    Box<dyn WriteEv>
);

impl Writer {
    pub(crate) fn new<P: AsRef<Path>>(
        outfile: P,
        format: Option<Format>,
    ) -> Result<Self> {
        let format = if let Some(format) = format {
            format
        } else {
            format_from_filename(&outfile)?
        };
        let inner: Box<dyn WriteEv> = match format {
            #[cfg(feature = "lhef")]
            Format::Lhef => Box::new(crate::lhef::Writer::new(outfile)?),
            #[cfg(feature = "hepmc2")]
            Format::HepMC2 => Box::new(crate::hepmc2::Writer::new(outfile)?),
            #[cfg(feature = "ntuple")]
            Format::NTuple => Box::new(crate::ntuple::Writer::new(outfile)?),
        };
        Ok(Self(inner))
    }
}

pub(crate) trait WriteEv {
    fn write(&mut self, event: Event) -> Result<()>;
}

impl WriteEv for Writer {
    fn write(&mut self, event: Event) -> Result<()> {
        self.0.write(event)
    }
}

fn format_from_filename<P: AsRef<Path>>(file: P) -> Result<Format> {
    let path = file.as_ref();
    let Some(suffix) = path.extension() else {
        bail!("No extension found in path {path:?}")
    };
    let Some(suffix) = suffix.to_str() else {
        bail!("Unknown file extension {suffix:?}")
    };
    use Format::*;
    let fmt = match suffix {
        #[cfg(feature = "hepmc2")]
        "hepmc" | "hepmc2" => HepMC2,
        #[cfg(feature = "lhef")]
        "lhe" | "lhef" => Lhef,
        #[cfg(feature = "ntuple")]
        "root" => NTuple,
        _ => bail!("Unknown file extension {suffix:?}")
    };
    Ok(fmt)
}
