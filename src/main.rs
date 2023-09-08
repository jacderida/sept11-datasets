use color_eyre::{eyre::eyre, Result};
use sept11_datasets::Release;
use std::path::PathBuf;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Reading releases. This can take several seconds...");
    let releases = Release::init_from_table(
        PathBuf::from("resources").join("release-table"),
        PathBuf::from("resources").join("torrents"),
    )?;
    for release in releases.iter() {
        println!("{release}");
    }

    let target_directory = PathBuf::from("/mnt/sept11-archive/9-11-archive/911datasets.org");
    let release = releases
        .iter()
        .find(|r| r.name == "NIST FOIA 09-42 - ic911studies.org - Release 28")
        .ok_or_else(|| eyre!("Could not find release"))?;
    let outcome = release.verify(target_directory)?;
    println!("{outcome}");
    match outcome {
        sept11_datasets::VerificationOutcome::Incomplete(missing_files, mismatched_files) => {
            println!("Missing files: {:#?}", missing_files);
            println!("Mismatched files: {:#?}", mismatched_files);
        }
        _ => {}
    }

    Ok(())
}
