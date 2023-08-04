fn main() {
    #[cfg(feature = "ntuple")]
    for flag in ntuple::ROOT_LINKER_FLAGS {
        println!("cargo:rustc-link-arg={flag}");
    }
}
