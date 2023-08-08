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
#[clap(version, about = "Convert events between different formats")]
struct Opt {
    /// Skip the first <SKIP> events
    #[clap(short, long)]
    skip: Option<usize>,

    /// Only convert the first <NUM> events
    #[clap(short, long)]
    num: Option<usize>,

    /// Input event file
    infile: PathBuf,

    /// Output event file
    outfile: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    let reader = Reader::new(&opt.infile).with_context(
        || format!("Failed to read from {:?}", opt.infile)
    )?;
    let mut writer = Writer::new(&opt.outfile).with_context(
        || format!("Failed to creat writer to {:?}", opt.outfile)
    )?;

    let skip = opt.skip.unwrap_or(0);
    let num = opt.num.unwrap_or(usize::MAX);

    for event in reader.skip(skip).take(num) {
        writer.write(event?)?;
    }
    // TODO:
    // writer.finish()?;

    Ok(())
}
