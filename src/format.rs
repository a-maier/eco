use clap::ValueEnum;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ValueEnum,
)]
#[clap(rename_all = "lower")]
pub(crate) enum Format {
    #[cfg(feature = "lhef")]
    Lhef,
    #[cfg(feature = "hepmc2")]
    HepMC2,
    #[cfg(feature = "ntuple")]
    NTuple,
}
