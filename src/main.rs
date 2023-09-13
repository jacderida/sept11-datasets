use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};
use sept11_datasets::db::*;
use sept11_datasets::{Release, VerificationOutcome};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "sept11-datasets", version = env!("CARGO_PKG_VERSION"))]
struct Opt {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build the release database from the torrent files
    Init {
        /// Path to the directory containing the release torrent files
        #[arg(short = 'n', long)]
        torrents_path: PathBuf,
    },
    // Print the releases
    Ls {
        /// Set to print the directory of the release rather than the name
        #[arg(long)]
        directory: bool,
    },
    // Print the current verification status releases
    Status {
        /// Display the status of a particular release.
        ///
        /// If not supplied, all releases will be iterated.
        #[arg(long)]
        release_id: Option<String>,
    },
    // Verify releases against their corresponding torrents
    Verify {
        /// The ID of the release to verify.
        ///
        /// If not supplied, all releases will be iterated.
        #[arg(long)]
        id: Option<String>,
        /// Path to the directory containing the files for the release
        #[arg(long)]
        target_path: PathBuf,
        /// Path to the directory containing the release torrent files
        #[arg(long)]
        torrents_path: PathBuf,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();
    match opt.command {
        Some(Commands::Init { torrents_path }) => {
            println!("Building releases from static data...");
            let releases = Release::init_releases(torrents_path)?;
            for release in releases.iter() {
                println!("{release}");
            }

            println!("Saving releases to new database...");
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            create_db_schema(&conn)?;
            for release in releases.iter() {
                save_release(&conn, &release)?;
            }
            let _ = conn.close();
            println!("Done");
            Ok(())
        }
        Some(Commands::Ls { directory }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let releases = get_releases(&conn)?;
            let _ = conn.close();
            for release in releases.iter() {
                if directory {
                    println!(
                        "{}: {}",
                        release.id,
                        release.directory.clone().unwrap_or("None".to_string())
                    )
                } else {
                    println!("{}: {}", release.id, release.name);
                }
            }
            Ok(())
        }
        Some(Commands::Status { release_id }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            if let Some(id) = release_id {
                let release = get_release_by_id(&conn, &id)?;
                release.print_verification_status()?;
            } else {
                let releases = get_releases(&conn)?;
                Release::print_status_table(&releases)?;
            }
            Ok(())
        }
        Some(Commands::Verify {
            id,
            target_path,
            torrents_path,
        }) => {
            // The release verification process can potentially take a very long time, so the
            // database connection will not be left open while that's running.
            // We'll open a new connection at the end of verification and use that to save the
            // result.
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            if let Some(id) = id {
                let mut release = get_release_by_id(&conn, &id)?;
                let _ = conn.close();
                verify_release(&mut release, &torrents_path, &target_path)?;
            } else {
                let mut releases = get_releases(&conn)?;
                let _ = conn.close();
                for mut release in releases.iter_mut() {
                    verify_release(&mut release, &torrents_path, &target_path)?;
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

fn verify_release(
    release: &mut Release,
    torrents_path: &PathBuf,
    target_path: &PathBuf,
) -> Result<()> {
    println!("Processing release: {}", release.name);
    let outcome = if let Some(verification_outcome) = &release.verification_outcome {
        println!("This release was previously verified");
        verification_outcome.clone()
    } else {
        println!("Performing verification...");
        let outcome = release.verify(torrents_path, target_path)?;
        release.verification_outcome = Some(outcome.clone());

        let mut conn = get_db_connection(get_database_path()?)?;
        save_verification_result(&mut conn, &release)?;
        let _ = conn.close();
        outcome
    };
    match outcome {
        VerificationOutcome::Incomplete(missing, corrupted) => {
            println!("Outcome: INCOMPLETE");
            println!("Missing files: {missing:#?}");
            println!("Corrupted files: {corrupted:#?}");
        }
        _ => {
            println!("Outcome: {}", outcome);
        }
    }
    Ok(())
}

fn get_database_path() -> Result<PathBuf> {
    let path = dirs_next::data_dir()
        .ok_or_else(|| eyre!("Could not retrieve data directory"))?
        .join("sept11-datasets");
    if !path.exists() {
        std::fs::create_dir_all(path.clone())?;
    }
    let db_path = path.join("releases.db");
    Ok(db_path)
}
