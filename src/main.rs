use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};
use dialoguer::Editor;
use sept11_datasets::db::*;
use sept11_datasets::{Release, VerificationOutcome};
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[clap(name = "sept11-datasets", version = env!("CARGO_PKG_VERSION"))]
struct Opt {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check the release for completeness by seeing if all the files are present and comparing the
    /// sizes against those in the torrent.
    ///
    /// This is to attempt to mark a release as complete, as opposed to verified. I had one release
    /// which could not be verified, but did not appear to have any corrupt files.
    Check {
        /// The ID of the release to download
        #[arg(long)]
        id: String,
        /// Path to the directory containing the files for the release
        #[arg(long)]
        target_path: PathBuf,
        /// Path to the directory containing the release torrent files
        #[arg(long)]
        torrents_path: PathBuf,
    },
    /// Download a release from the Internet Archive.
    ///
    /// Some releases are on the archive, where they have the same tree as the torrent. Given the
    /// URL prefix of the release on the archive, we can use the information from the torrent to
    /// download all the files individually.
    ///
    /// To avoid abuse of the archive, the files are downloaded sequentially.
    #[clap(name = "download-release", verbatim_doc_comment)]
    DownloadRelease {
        /// The ID of the release to download
        #[arg(long)]
        id: String,
        /// The URL of the release on the archive.
        ///
        /// This should be the top level URL, e.g., https://archive.org/download/NIST_9-11_Release_01.
        #[arg(long, verbatim_doc_comment)]
        url: String,
        /// Path specifying where the files should be downloaded
        #[arg(long)]
        target_path: PathBuf,
        /// Path to the directory containing the release torrent files
        #[arg(long)]
        torrents_path: PathBuf,
    },
    /// Build the release database from the torrent files.
    ///
    /// If the database already exists, running this command again will add any new schema that
    /// needs to be created.
    Init {
        /// Path to the directory containing the release torrent files
        #[arg(long)]
        torrents_path: Option<PathBuf>,
    },
    // Print the releases
    Ls {
        /// Set to print the directory of the release rather than the name
        #[arg(long)]
        directory: bool,
    },
    // List the files in the torrent
    #[clap(name = "ls-files", verbatim_doc_comment)]
    LsFiles {
        /// The id of the release
        #[arg(long)]
        id: String,
        /// Path to the directory containing the release torrent files
        #[arg(long)]
        torrents_path: PathBuf,
    },
    // Mark a release as incomplete.
    //
    // Provide the list of missing or corrupt files by pointing to a text file, where each line in
    // the file is a path.
    #[clap(name = "mark-incomplete", verbatim_doc_comment)]
    MarkIncomplete {
        /// The id of the release
        #[arg(long)]
        id: String,
        /// Path to a file containing a list of missing files for the release
        #[arg(long)]
        missing_files_path: Option<PathBuf>,
        /// Path to a file containing a list of corrupt files for the release
        #[arg(long)]
        corrupt_files_path: Option<PathBuf>,
    },
    /// Add or edit notes for a release.
    ///
    /// Set the EDITOR variable to determine which editor will be used to compose the note.
    Notes {
        /// The id of the release
        #[arg(long)]
        id: String,
    },
    // Reset verification result for releases
    Reset {
        /// Only reset the release with the specified ID.
        ///
        /// If not supplied, all missing releases will be reset.
        #[arg(long)]
        id: Option<String>,
    },
    // Print the current verification status releases
    Status {
        /// Display the status of a particular release.
        ///
        /// If not supplied, all releases will be iterated.
        #[arg(long)]
        id: Option<String>,
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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();
    match opt.command {
        Some(Commands::Check {
            id,
            target_path,
            torrents_path,
        }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let mut release = get_release_by_id(&conn, &id)?;
            let _ = conn.close();
            let outcome = if let Some(verification_outcome) = &release.verification_outcome {
                println!("This release was previously verified");
                verification_outcome.clone()
            } else {
                let outcome = release.check(&torrents_path, &target_path)?;
                release.verification_outcome = Some(outcome.clone());
                let mut conn = get_db_connection(get_database_path()?)?;
                save_verification_result(&mut conn, &release)?;
                let _ = conn.close();
                outcome
            };
            match outcome {
                VerificationOutcome::Incomplete(missing, corrupted) => {
                    println!("Outcome: INCOMPLETE");
                    if !missing.is_empty() {
                        println!("Missing files:");
                        for file in missing.iter() {
                            println!("{}", file.to_string_lossy());
                        }
                    } else {
                        println!("Files with size mismatch:");
                        for file in corrupted.iter() {
                            println!("{}", file.to_string_lossy());
                        }
                    }
                }
                _ => {
                    println!("Outcome: {}", outcome);
                }
            }
            Ok(())
        }
        Some(Commands::DownloadRelease {
            id,
            url,
            target_path,
            torrents_path,
        }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let release = get_release_by_id(&conn, &id)?;
            let _ = conn.close();
            let url = Url::parse(&url)?;
            release
                .download_release_from_archive(&url, &torrents_path, &target_path)
                .await?;
            Ok(())
        }
        Some(Commands::Init { torrents_path }) => {
            let db_path = get_database_path()?;
            if db_path.exists() {
                let conn = get_db_connection(&db_path)?;
                create_db_schema(&conn)?;
                println!("Updated database schema");
                return Ok(());
            }

            println!("Building releases from static data...");
            let releases = Release::init_releases(torrents_path.ok_or_else(|| {
                eyre!(
                "When creating the database for the first time, the --torrents-path argument must \
                be supplied")
            })?)?;
            for release in releases.iter() {
                println!("{release}");
            }

            println!("Saving releases to new database...");
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            create_db_schema(&conn)?;
            for release in releases.iter() {
                save_new_release(&conn, &release)?;
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
                        "{}: {} ({} files)",
                        release.id,
                        release.directory.clone().unwrap_or("None".to_string()),
                        release.file_count.unwrap_or(0),
                    )
                } else {
                    println!(
                        "{}: {} ({} files)",
                        release.id,
                        release.name,
                        release.file_count.unwrap_or(0)
                    );
                }
            }
            Ok(())
        }
        Some(Commands::LsFiles { id, torrents_path }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let release = get_release_by_id(&conn, &id)?;
            let files = release.get_torrent_tree(&torrents_path)?;
            for file in files.iter() {
                println!("{}", file.to_string_lossy());
            }
            Ok(())
        }
        Some(Commands::MarkIncomplete {
            id,
            missing_files_path,
            corrupt_files_path,
        }) => {
            let db_path = get_database_path()?;
            let mut conn = get_db_connection(&db_path)?;
            let mut release = get_release_by_id(&conn, &id)?;
            release
                .mark_incomplete(missing_files_path.as_deref(), corrupt_files_path.as_deref())?;
            save_verification_result(&mut conn, &mut release)?;
            println!("Marked {} as incomplete", release.name);
            Ok(())
        }
        Some(Commands::Notes { id }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let release = get_release_by_id(&conn, &id)?;
            if let Some(edited_notes) = Editor::new()
                .edit(&release.notes.unwrap_or("".to_string()))
                .unwrap()
            {
                save_notes(&conn, &id, &edited_notes)?;
            }
            Ok(())
        }
        Some(Commands::Reset { id }) => {
            let db_path = get_database_path()?;
            let mut conn = get_db_connection(&db_path)?;
            if let Some(id) = id {
                reset_verification_result(&mut conn, &id)?;
                println!("Set {} back to UNKNOWN status", id);
            } else {
                let releases = get_missing_releases(&conn)?;
                for release in releases.iter() {
                    reset_verification_result(&mut conn, &release.id)?;
                    println!("Set {} back to UNKNOWN status", release.name);
                }
            }
            Ok(())
        }
        Some(Commands::Status { id }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            if let Some(id) = id {
                let release = get_release_by_id(&conn, &id)?;
                release.print_verification_status()?;
            } else {
                let releases = get_releases(&conn)?;
                Release::print_status_table(&releases)?;
                let bytes_remaining: u64 = releases
                    .iter()
                    .filter(|x| x.get_verification_outcome() == "MISSING")
                    .map(|x| x.size.unwrap_or(0))
                    .sum();
                let human_readable_bytes_remaining =
                    sept11_datasets::bytes_to_human_readable(bytes_remaining);
                println!("{human_readable_bytes_remaining} still missing");
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
