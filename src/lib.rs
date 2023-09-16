pub mod db;
pub mod error;
pub mod release_data;

use crate::error::{Error, Result};
use crate::release_data::RELEASE_DATA;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use lava_torrent::torrent::v1::Torrent;
use prettytable::{color, Attr, Cell, Row as TableRow, Table};
use rusqlite::Row;
use sha1::{Digest, Sha1};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;
use url::Url;

const WRAP_LENGTH: usize = 72;

#[derive(Clone)]
pub enum VerificationOutcome {
    Verified,
    TorrentMissing,
    Incomplete(Vec<PathBuf>, Vec<String>),
    AllFilesMissing,
}

impl fmt::Display for VerificationOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationOutcome::Verified => write!(f, "VERIFIED"),
            VerificationOutcome::TorrentMissing => write!(f, "NO TORRENT"),
            VerificationOutcome::Incomplete(_, _) => write!(f, "INCOMPLETE"),
            VerificationOutcome::AllFilesMissing => write!(f, "MISSING"),
        }
    }
}

pub struct Release {
    pub id: String,
    pub date: String,
    pub name: String,
    pub directory: Option<String>,
    pub file_count: Option<usize>,
    pub size: Option<u64>,
    pub torrent_url: Url,
    pub verification_outcome: Option<VerificationOutcome>,
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({} files, {})",
            self.date,
            self.name,
            self.file_count
                .map_or("None".to_string(), |n| n.to_string()),
            self.size
                .map_or("None".to_string(), |s| bytes_to_human_readable(s))
        )
    }
}

impl Release {
    pub fn new(
        date: String,
        name: String,
        directory: Option<String>,
        file_count: Option<usize>,
        size: Option<u64>,
        torrent_url: Url,
    ) -> Self {
        let id = Release::generate_id(&date, &name);
        Self {
            id,
            date,
            name,
            directory,
            file_count,
            size,
            torrent_url,
            verification_outcome: None,
        }
    }

    pub fn print_status_table(releases: &Vec<Release>) -> Result<()> {
        let mut table = Table::new();
        for release in releases.iter() {
            let wrapped_title = textwrap::wrap(&release.name, WRAP_LENGTH).join("\n");
            let outcome_cell = match release.verification_outcome.as_ref() {
                Some(outcome) => match outcome {
                    VerificationOutcome::Verified => Cell::new("VERIFIED")
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
                    VerificationOutcome::TorrentMissing => Cell::new("TORRENT MISSING")
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::YELLOW)),
                    VerificationOutcome::Incomplete(_, _) => Cell::new("INCOMPLETE")
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::CYAN)),
                    VerificationOutcome::AllFilesMissing => Cell::new("MISSING")
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::RED)),
                },
                None => Cell::new("UNKNOWN")
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::RED)),
            };
            table.add_row(TableRow::new(vec![
                Cell::new(&release.date),
                Cell::new(&wrapped_title),
                outcome_cell,
            ]));
        }

        table.printstd();
        Ok(())
    }

    pub fn get_verification_outcome(&self) -> String {
        if let Some(outcome) = self.verification_outcome.as_ref() {
            outcome.to_string()
        } else {
            "UNKNOWN".to_string()
        }
    }

    pub fn print_verification_status(&self) -> Result<()> {
        println!("{}", self.name);
        match &self.verification_outcome {
            Some(VerificationOutcome::Verified) => {
                let file_count = self.file_count.ok_or_else(|| {
                    Error::VerificationReportError(
                        "a verified release must have a file count".to_string(),
                    )
                })?;
                println!("Status: {}", "VERIFIED".bright_green());
                println!(
                    "All {} files were verified against the torrent piece hashes",
                    file_count
                );
            }
            Some(VerificationOutcome::Incomplete(missing_files, corrupt_files)) => {
                println!("Status: {}", "VERIFIED".cyan());
                let file_count = self.file_count.ok_or_else(|| {
                    Error::VerificationReportError(
                        "an incomplete release must have a file count".to_string(),
                    )
                })?;
                if missing_files.len() > 0 {
                    println!("{missing_files:#?}");
                    println!(
                        "{} of {} files were missing from this release",
                        missing_files.len(),
                        file_count,
                    );
                }
                if corrupt_files.len() > 0 {
                    println!("{corrupt_files:#?}");
                    println!(
                        "{} of {} files are corrupted for this release",
                        missing_files.len(),
                        file_count,
                    );
                }
            }
            Some(VerificationOutcome::TorrentMissing) => {
                println!("Status: {}", "TORRENT MISSING".yellow());
            }
            Some(VerificationOutcome::AllFilesMissing) => {
                let file_count = self.file_count.ok_or_else(|| {
                    Error::VerificationReportError(
                        "an incomplete release must have a file count".to_string(),
                    )
                })?;
                println!("Status: {}", "MISSING".red());
                println!("All {} files were missing for this release", file_count);
            }
            None => println!("Status: {}", "UNKNOWN".red()),
        }
        Ok(())
    }

    pub fn generate_id(date: &str, name: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(date.as_bytes());
        hasher.update(name.as_bytes());
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }

    pub fn from_row(
        row: &Row,
        missing_files: &Vec<PathBuf>,
        corrupt_files: &Vec<String>,
    ) -> Result<Release> {
        let id: String = row.get(0)?;
        let date: String = row.get(1)?;
        let name: String = row.get(2)?;
        let directory: Option<String> = row.get(3)?;
        let file_count: Option<usize> = row.get(4)?;
        let size: Option<u64> = row.get(5)?;
        let torrent_url: String = row.get(6)?;
        let torrent_url = Url::parse(&torrent_url).unwrap();
        let verification_outcome: String = row.get(7)?;
        let verification_outcome = match verification_outcome.as_str() {
            "VERIFIED" => Some(VerificationOutcome::Verified),
            "NO TORRENT" => Some(VerificationOutcome::TorrentMissing),
            "MISSING" => Some(VerificationOutcome::AllFilesMissing),
            "INCOMPLETE" => Some(VerificationOutcome::Incomplete(
                missing_files.clone(),
                corrupt_files.clone(),
            )),
            "UNKNOWN" => None,
            _ => None,
        };
        Ok(Release {
            id,
            date,
            name,
            directory,
            file_count,
            size,
            torrent_url,
            verification_outcome,
        })
    }

    pub fn init_releases(torrents_path: PathBuf) -> Result<Vec<Release>> {
        let mut releases = Vec::new();
        for item in RELEASE_DATA.iter() {
            let date = item.0.to_string();
            let torrent_url = item.1.to_string();
            let name = item.2.to_string();

            let url = Url::parse(&torrent_url)?;
            let file_name = Self::get_file_name_from_url(&url)?;
            let torrent_path = torrents_path.join(file_name);
            let (directory, file_count, size) = if torrent_path.exists() {
                match Torrent::read_from_file(torrent_path.clone()) {
                    Ok(torrent) => {
                        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;
                        let first_file = &files[0];
                        // We want to store the directory below '911datasets.org'.
                        let directory: String = {
                            let mut ancestors = first_file.path.ancestors();
                            let mut second_to_last = None;
                            let mut last = ancestors.next();
                            while let Some(current) = ancestors.next() {
                                second_to_last = last;
                                last = Some(current);
                            }
                            second_to_last
                                .map(|p| p.to_path_buf())
                                .ok_or_else(|| Error::ReleaseDirectoryNotObtained)?
                                .to_string_lossy()
                                .to_string()
                        };
                        let mut size = 0;
                        for file in files.iter() {
                            size += file.length;
                        }
                        (Some(directory), Some(files.len()), Some(size as u64))
                    }
                    Err(_) => (None, None, None),
                }
            } else {
                (None, None, None)
            };
            releases.push(Release::new(date, name, directory, file_count, size, url));
        }
        Ok(releases)
    }

    pub fn verify(
        &self,
        torrents_path: &PathBuf,
        target_directory: &PathBuf,
    ) -> Result<VerificationOutcome> {
        let mut mismatched_files = HashSet::new();
        let mut missing_files = HashSet::new();

        if self.file_count.is_none() {
            return Ok(VerificationOutcome::TorrentMissing);
        }

        let torrent_path = torrents_path.join(Self::get_file_name_from_url(&self.torrent_url)?);
        let torrent = Torrent::read_from_file(torrent_path)?;
        let piece_length = torrent.piece_length;
        let num_pieces = torrent.pieces.len();
        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;

        // Check for a 'MISSING' result first. This is quicker than going through all the pieces.
        println!("Checking first to see if all files are missing...");
        let mut missing_count = 0;
        for file in files.iter() {
            let path = target_directory.join(file.path.clone());
            if !path.exists() {
                missing_count += 1;
            }
        }
        if missing_count == files.len() {
            return Ok(VerificationOutcome::AllFilesMissing);
        }

        println!("There are files available for this release. Will now verify.");
        println!("The torrent has {} pieces to verify", num_pieces);
        let bar = ProgressBar::new(num_pieces as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")?
                .progress_chars("#>-"),
        );

        let mut piece_idx = 0;
        let mut file_idx = 0;
        let mut offset: u64 = 0;

        while piece_idx < num_pieces {
            let piece_hash = &torrent.pieces[piece_idx];
            let mut buffer = Vec::new();
            let mut is_missing_file = false;

            // This loop reads a piece, which can span across multiple files, if the files are
            // smaller than the piece size. The 'pieces' in the torrent file are with respect to
            // the whole content rather than any individual file.
            while buffer.len() < piece_length as usize {
                let file_info = &files[file_idx];
                let file_path = target_directory.join(file_info.path.clone());

                if !file_path.exists() {
                    missing_files.insert(file_info.path.clone());
                    is_missing_file = true;

                    let remaining_in_file = file_info.length as u64 - offset;
                    let remaining_in_piece = (piece_length as usize) - buffer.len();
                    let skip = std::cmp::min(remaining_in_file, remaining_in_piece as u64);

                    offset += skip;
                    buffer.resize(buffer.len() + skip as usize, 0);

                    if offset >= file_info.length as u64 {
                        offset = 0;
                        file_idx += 1;
                        if file_idx == files.len() {
                            break;
                        }
                    }
                    continue;
                }

                let mut file = File::open(file_path)?;
                file.seek(std::io::SeekFrom::Start(offset))?;

                let remaining = (piece_length as usize) - buffer.len();
                let mut temp_buf = vec![0; remaining];
                let read_bytes_len = file.read(&mut temp_buf)?;

                buffer.extend_from_slice(&temp_buf[0..read_bytes_len]);

                if ((file_info.length as u64) - offset) as usize <= remaining {
                    offset = 0;
                    file_idx += 1;
                    if file_idx == files.len() {
                        break;
                    }
                } else {
                    offset += read_bytes_len as u64;
                }
            }

            // Now that the whole piece has been read, verify it.
            let mut hasher = Sha1::new();
            hasher.update(&buffer);
            let result = hasher.finalize();
            if result.as_slice() != piece_hash && !is_missing_file {
                if file_idx == files.len() {
                    break;
                }
                mismatched_files.insert(files[file_idx].path.to_string_lossy().to_string());
            }

            piece_idx += 1;
            bar.inc(1);
        }

        if !missing_files.is_empty() || !mismatched_files.is_empty() {
            return Ok(VerificationOutcome::Incomplete(
                missing_files.into_iter().collect(),
                mismatched_files.into_iter().collect(),
            ));
        }

        Ok(VerificationOutcome::Verified)
    }

    fn get_file_name_from_url(url: &Url) -> Result<String> {
        let file_name = url
            .path_segments()
            .ok_or(Error::PathSegmentsParseError)?
            .last()
            .ok_or(Error::PathSegmentsParseError)?;
        Ok(file_name.to_string())
    }
}

pub fn bytes_to_human_readable(bytes: u64) -> String {
    const TB: u64 = 1024 * 1024 * 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    }
}
