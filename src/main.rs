mod reader;
mod writer;
#[cfg(feature = "lhef")]
mod lhef;
#[cfg(feature = "hepmc2")]
mod hepmc2;
#[cfg(feature = "ntuple")]
mod ntuple;

use std::path::PathBuf;

use anyhow::{Result, Context};
use clap::Parser;

use crate::{writer::{WriteEv, Writer}, reader::Reader};

#[derive(Debug, Parser)]
#[clap(version)]
struct Opt {
    pub(crate) infile: PathBuf,

    pub(crate) outfile: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    let reader = Reader::new(&opt.infile).with_context(
        || format!("Failed to read from {:?}", opt.infile)
    )?;
    let mut writer = Writer::new(&opt.outfile).with_context(
        || format!("Failed to creat writer to {:?}", opt.outfile)
    )?;

    for event in reader {
        writer.write(event?)?;
    }
    // TODO:
    // writer.finish()?;

    Ok(())
}
