mod reader;
mod writer;
#[cfg(feature = "lhef")]
mod lhef;
#[cfg(feature = "hepmc2")]
mod hepmc2;
#[cfg(feature = "ntuple")]
mod ntuple;

use anyhow::{Result, bail, Context};

use crate::{writer::{WriteEv, Writer}, reader::Reader};

const USAGE: &str = "eco INFILE OUTFILE";

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(infile) = args.next() else {
        bail!("INFILE argument missing: \nUsage: {USAGE}")
    };
    let Some(outfile) = args.next() else {
        bail!("OUTFILE argument missing: \nUsage: {USAGE}")
    };

    let reader = Reader::new(&infile).with_context(
        || format!("Failed to read from {infile}")
    )?;
    let mut writer = Writer::new(&outfile).with_context(
        || format!("Failed to creat writer to {outfile}")
    )?;

    for event in reader {
        writer.write(event?)?;
    }
    // TODO:
    // writer.finish()?;

    Ok(())
}
