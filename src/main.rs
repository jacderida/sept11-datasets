use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};
use dialoguer::Editor;
use sept11_datasets::db::*;
use sept11_datasets::error::Error;
use sept11_datasets::{
    build_partial_release_11_from_nist_202, build_release_14_file_links, build_release_14_links,
    bytes_to_human_readable, download_torrents, Release, VerificationOutcome,
};
use std::path::PathBuf;
use tempdir::TempDir;

const RELEASE_14_UNCOMPRESSED_ID: &str = "968d5cdf934f01bb9efcf631c999fde5a617f4a9";
const RELEASE_11_ID: &str = "8bf4862b4238c79fc6aa039dab0b5b3a1915af64";

#[derive(Parser, Debug)]
#[clap(name = "sept11-datasets", version = env!("CARGO_PKG_VERSION"))]
struct Opt {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Copy release 11 files from the NIST 10 202 release.
    ///
    /// Many files that were in the NIST FOIA 10 202 are duplicates of what was in release 11.
    /// However, not all of the files are present.
    ///
    /// This command can copy the files from NIST FOIA 10 202 to NIST FOIA Release 11 to get a
    /// mostly complete release 11.
    #[clap(name = "build-partial-release-11", verbatim_doc_comment)]
    BuildPartialRelease11 {
        /// Path to the directory containing the files for the release
        #[arg(long, env = "DATASETS_PATH")]
        target_path: PathBuf,
    },
    /// Check all files are present and the sizes match those in the torrent
    #[clap(name = "check")]
    Check {
        /// The ID of the release to download
        #[arg(long)]
        id: String,
        /// Path to the directory containing the files for the release
        #[arg(long, env = "DATASETS_PATH")]
        target_path: PathBuf,
    },
    /// Download a release from the Internet Archive
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
        /// Path specifying where the files should be downloaded
        #[arg(long, env = "DATASETS_PATH")]
        target_path: PathBuf,
    },
    /// Build the release database from the static data in the binary
    ///
    /// The torrents are downloaded during this process.
    ///
    /// If the database already exists, running this command again will add any new schema that
    /// needs to be created.
    Init {},
    /// Print the list of releases
    Ls {
        /// Set to print the directory of the release rather than the name
        #[arg(long)]
        directory: bool,
    },
    /// List the files for the release
    #[clap(name = "ls-files", verbatim_doc_comment)]
    LsFiles {
        /// The id of the release
        #[arg(long)]
        id: String,
        /// Display file sizes in bytes rather than human readable text
        #[arg(long)]
        use_bytes: bool,
    },
    /// Mark a release as incomplete
    ///
    /// Provide the list of missing or corrupt files by pointing to a text file, where each line in
    /// the file is a path.
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
    /// Mark a release as missing
    ///
    /// Provide the list of missing or corrupt files by pointing to a text file, where each line in
    /// the file is a path.
    #[clap(name = "mark-missing", verbatim_doc_comment)]
    MarkMissing {
        /// The id of the release
        #[arg(long)]
        id: String,
    },
    /// Add or edit notes for a release
    ///
    /// Set the EDITOR variable to determine which editor will be used to compose the note.
    Notes {
        /// The id of the release
        #[arg(long)]
        id: String,
    },
    /// Reset the verification result for a release
    Reset {
        /// Only reset the release with the specified ID.
        ///
        /// If not supplied, all missing releases will be reset.
        #[arg(long)]
        id: Option<String>,
    },
    /// Print the current verification status for releases
    Status {
        /// Display the status of a particular release.
        ///
        /// If not supplied, all releases will be iterated.
        #[arg(long)]
        id: Option<String>,
        /// Show the list of any missing or corrupt files
        #[arg(long)]
        show_incomplete: bool,
    },
    /// Verify releases against their corresponding torrents
    Verify {
        /// The ID of the release to verify.
        ///
        /// If not supplied, all releases will be iterated.
        #[arg(long)]
        id: Option<String>,
        /// Path to the directory containing the files for the release
        #[arg(long, env = "DATASETS_PATH")]
        target_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();
    match opt.command {
        Some(Commands::BuildPartialRelease11 { target_path }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let release11 = get_release_by_id(&conn, &RELEASE_11_ID)?;
            let _ = conn.close();
            build_partial_release_11_from_nist_202(&release11, &target_path)?;
            Ok(())
        }
        Some(Commands::Check { id, target_path }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let mut release = get_release_by_id(&conn, &id)?;
            let _ = conn.close();
            let outcome = if let Some(verification_outcome) = &release.verification_outcome {
                println!("This release was previously verified");
                verification_outcome.clone()
            } else {
                let outcome = release.check(&target_path)?;
                release.verification_outcome = Some(outcome.clone());
                let mut conn = get_db_connection(get_database_path()?)?;
                save_verification_result(&mut conn, &release)?;
                let _ = conn.close();
                outcome
            };
            match outcome {
                VerificationOutcome::Incomplete(missing, size_mismatches) => {
                    println!("Outcome: INCOMPLETE");
                    if !missing.is_empty() {
                        println!("Missing {} files", missing.len());
                    }
                    if !size_mismatches.is_empty() {
                        println!("{} size mismatches", size_mismatches.len());
                    }
                }
                _ => {
                    println!("Outcome: {}", outcome);
                }
            }
            Ok(())
        }
        Some(Commands::DownloadRelease { id, target_path }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let release = get_release_by_id(&conn, &id)?;
            if id == RELEASE_14_UNCOMPRESSED_ID {
                // Release 14 is a special case for which the data is scattered across different
                // collections.
                let release_14_links = get_release_14_links(&conn)?;
                let release_14_file_links = get_release_14_file_links(&conn)?;
                let _ = conn.close();
                release
                    .download_release_14_from_archive(
                        release_14_links,
                        release_14_file_links,
                        &target_path,
                    )
                    .await?;
                return Ok(());
            }
            let _ = conn.close();

            let url = if let Some(url) = release.download_url.as_ref() {
                url
            } else {
                return Err(eyre!("This release does not have a download URL"));
            };
            let filename = url
                .path_segments()
                .and_then(|s| s.last())
                .ok_or_else(|| Error::FilenameFromUrlError)?;
            if PathBuf::from(filename).extension().unwrap() == "zip" {
                release
                    .download_zip_release_from_archive(&url, &target_path)
                    .await?;
            } else {
                release
                    .download_release_from_archive(&url, &target_path)
                    .await?;
            }
            Ok(())
        }
        Some(Commands::Init {}) => {
            let db_path = get_database_path()?;
            if db_path.exists() {
                let conn = get_db_connection(&db_path)?;
                create_db_schema(&conn)?;
                println!("Updated database schema");
                let _ = conn.close();

                let conn = get_db_connection(&db_path)?;
                let temp_dir = TempDir::new("torrents")?;
                download_torrents(&conn, &temp_dir.into_path()).await?;

                // The purpose of this is to save any additional data that was added to the static
                // release data. It should leave verification results unchanged.
                println!("Reinitialising release data...");
                let mut releases = get_releases(&conn)?;
                Release::reinit_releases(&mut releases)?;
                let count = releases.len();
                let mut i = 1;
                for release in releases.iter() {
                    println!("Saving release {} of {}", i, count);
                    save_release(&conn, &release)?;
                    i += 1;
                }
                let _ = conn.close();

                println!("Building release 14 collection links...");
                let conn = get_db_connection(&db_path)?;
                let release_14 = get_release_by_id(&conn, RELEASE_14_UNCOMPRESSED_ID)?;
                build_release_14_links(&conn, &release_14)?;
                let _ = conn.close();

                println!("Building release 14 file links...");
                let conn = get_db_connection(&db_path)?;
                build_release_14_file_links(&conn)?;
                let _ = conn.close();

                return Ok(());
            }

            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            create_db_schema(&conn)?;

            let temp_dir = TempDir::new("torrents")?;
            let temp_path = temp_dir.into_path().clone();
            download_torrents(&conn, &temp_path).await?;

            println!("Building releases from static data...");
            let releases = Release::init_releases(temp_path)?;
            for release in releases.iter() {
                println!("{release}");
            }

            println!("Saving releases...");
            for release in releases.iter() {
                save_release(&conn, &release)?;
            }
            let _ = conn.close();
            println!("Done");

            println!("Building release 14 links...");
            let conn = get_db_connection(&db_path)?;
            let release_14 = get_release_by_id(&conn, RELEASE_14_UNCOMPRESSED_ID)?;
            build_release_14_links(&conn, &release_14)?;
            let _ = conn.close();

            println!("Building release 14 file links...");
            let conn = get_db_connection(&db_path)?;
            build_release_14_file_links(&conn)?;
            let _ = conn.close();

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
        Some(Commands::LsFiles { id, use_bytes }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            let release = get_release_by_id(&conn, &id)?;
            let files = release.get_torrent_tree()?;
            for (path, size) in files.iter() {
                if use_bytes {
                    println!("{} ({})", path.to_string_lossy(), *size);
                } else {
                    println!(
                        "{} ({})",
                        path.to_string_lossy(),
                        bytes_to_human_readable(*size)
                    );
                }
            }
            let _ = conn.close();
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
        Some(Commands::MarkMissing { id }) => {
            let db_path = get_database_path()?;
            let mut conn = get_db_connection(&db_path)?;
            let mut release = get_release_by_id(&conn, &id)?;
            release.mark_missing()?;
            save_verification_result(&mut conn, &mut release)?;
            println!("Marked {} as missing", release.name);
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
        Some(Commands::Status {
            id,
            show_incomplete,
        }) => {
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            if let Some(id) = id {
                let release = get_release_by_id(&conn, &id)?;
                release.print_verification_status(show_incomplete)?;
            } else {
                let releases = get_releases(&conn)?;
                Release::print_status_table(&releases)?;
                let missing_bytes_remaining: u64 = releases
                    .iter()
                    .filter(|x| x.get_verification_outcome() == "MISSING")
                    .map(|x| x.size.unwrap_or(0))
                    .sum();
                let incomplete_bytes_remaining: u64 = releases
                    .iter()
                    .filter(|x| x.get_verification_outcome() == "INCOMPLETE")
                    .map(|x| {
                        let size = if let Some(outcome) = &x.verification_outcome {
                            match outcome {
                                VerificationOutcome::Incomplete(missing, size_mismatch) => {
                                    let missing_size: u64 = missing.iter().map(|x| x.1).sum();
                                    let mismatch_size: u64 =
                                        size_mismatch.iter().map(|x| x.1).sum();
                                    missing_size + mismatch_size
                                }
                                _ => 0,
                            }
                        } else {
                            0
                        };
                        size
                    })
                    .sum();
                if missing_bytes_remaining > 0 || incomplete_bytes_remaining > 0 {
                    println!(
                        "Missing: {}",
                        sept11_datasets::bytes_to_human_readable(missing_bytes_remaining)
                    );
                    println!(
                        "Incomplete/Corrupt: {}",
                        sept11_datasets::bytes_to_human_readable(incomplete_bytes_remaining)
                    );
                    let total_bytes_missing = missing_bytes_remaining + incomplete_bytes_remaining;
                    let human_readable_bytes_remaining =
                        sept11_datasets::bytes_to_human_readable(total_bytes_missing);
                    println!("Total: {human_readable_bytes_remaining} still missing");
                }
            }
            Ok(())
        }
        Some(Commands::Verify { id, target_path }) => {
            // The release verification process can potentially take a very long time, so the
            // database connection will not be left open while that's running.
            // We'll open a new connection at the end of verification and use that to save the
            // result.
            let db_path = get_database_path()?;
            let conn = get_db_connection(&db_path)?;
            if let Some(id) = id {
                let mut release = get_release_by_id(&conn, &id)?;
                let _ = conn.close();
                verify_release(&mut release, &target_path)?;
            } else {
                let mut releases = get_releases(&conn)?;
                let _ = conn.close();
                for mut release in releases.iter_mut() {
                    verify_release(&mut release, &target_path)?;
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

fn verify_release(release: &mut Release, target_path: &PathBuf) -> Result<()> {
    println!("Processing release: {}", release.name);
    let outcome = if let Some(verification_outcome) = &release.verification_outcome {
        println!("This release was previously verified");
        verification_outcome.clone()
    } else {
        let outcome = release.verify(target_path)?;
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
