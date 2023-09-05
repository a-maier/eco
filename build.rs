use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(feature = "ntuple")]
    // work around https://github.com/rust-lang/cargo/issues/12326
    // once this is fixed, we can remove the code below and use
    // ```
    // for flag in ntuple::ROOT_LINKER_FLAGS {
    //     println!("cargo:rustc-link-arg={flag}");
    // }
    // ```
    for flag in get_root_flags("--libs")? {
        println!("cargo:rustc-link-arg={flag}");
    }
    Ok(())
}

#[cfg(feature = "ntuple")]
fn get_root_flags(flags: &str) -> Result<Vec<String>> {
    use anyhow::{bail, Context};
    use std::{process::Command, str::from_utf8};

    const CFG_CMD: &str = "root-config";

    let cmd = format!("{CFG_CMD} {flags}");
    let output = Command::new(CFG_CMD)
        .arg(flags)
        .output()
        .with_context(|| format!("Failed to run `{cmd}`"))?;
    if !output.status.success() {
        if output.stderr.is_empty() {
            bail!("{CFG_CMD} {flags} failed without error messages");
        } else {
            bail!(
                "{CFG_CMD} {flags} failed: {}",
                from_utf8(&output.stderr).unwrap()
            );
        }
    }
    let args = from_utf8(&output.stdout)
        .with_context(|| format!("Failed to convert `{cmd}` output to utf8"))?;
    Ok(args.split_whitespace().map(|arg| arg.to_owned()).collect())
}
