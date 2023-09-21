pub mod db;
pub mod error;
pub mod release_data;

use crate::error::{Error, Result};
use crate::release_data::RELEASE_DATA;
use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lava_torrent::torrent::v1::Torrent;
use prettytable::{color, Attr, Cell, Row as TableRow, Table};
use rusqlite::Row;
use sha1::{Digest, Sha1};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::time::{sleep, Duration};
use url::Url;

const WRAP_LENGTH: usize = 72;

#[derive(Clone)]
pub enum VerificationOutcome {
    Complete,
    Verified,
    TorrentMissing,
    Incomplete(Vec<PathBuf>, Vec<String>),
    AllFilesMissing,
}

impl fmt::Display for VerificationOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationOutcome::Complete => write!(f, "COMPLETE"),
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
            let title = match release.verification_outcome {
                Some(VerificationOutcome::TorrentMissing) => release.name.clone(),
                Some(_) => {
                    format!(
                        "{} -- {} files -- {}",
                        release.name,
                        release.file_count.unwrap(),
                        bytes_to_human_readable(release.size.unwrap())
                    )
                }
                None => release.name.clone(),
            };
            let wrapped_title = textwrap::wrap(&title, WRAP_LENGTH).join("\n");
            let outcome_cell = match release.verification_outcome.as_ref() {
                Some(outcome) => match outcome {
                    VerificationOutcome::Complete => Cell::new("COMPLETE")
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
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
            Some(VerificationOutcome::Complete) => {
                println!("Status: {}", "COMPLETE".bright_green());
                println!(
                    "All files are present, but hashes don't match, or there was no torrent to \
                        compare the release with.",
                );
            }
            Some(VerificationOutcome::Incomplete(missing_files, corrupt_files)) => {
                println!("Status: {}", "INCOMPLETE".cyan());
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
                        corrupt_files.len(),
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
            "COMPLETE" => Some(VerificationOutcome::Complete),
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

    pub fn get_torrent_tree(&self, torrents_path: &PathBuf) -> Result<Vec<PathBuf>> {
        let torrent_path = torrents_path.join(Self::get_file_name_from_url(&self.torrent_url)?);
        let torrent = Torrent::read_from_file(torrent_path)?;
        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;
        let tree = files
            .iter()
            .map(|f| f.path.clone())
            .collect::<Vec<PathBuf>>();
        Ok(tree)
    }

    pub async fn download_release_from_archive(
        &self,
        base_url: &Url,
        torrents_path: &PathBuf,
        base_target_path: &PathBuf,
    ) -> Result<()> {
        let tree = self.get_torrent_tree(&torrents_path)?;
        let multi_progress = MultiProgress::new();
        let total_pb = multi_progress.add(ProgressBar::new(tree.len() as u64));
        total_pb.set_style(
            ProgressStyle::default_bar()
                .template("Overall progress: [{bar:40.cyan/blue}] {pos}/{len} files")?
                .progress_chars("#>-"),
        );

        let file_pb = multi_progress.add(ProgressBar::new(0));
        file_pb.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{bar:30.green/blue}] {bytes}/{total_bytes} {bytes_per_sec}")?
                .progress_chars("=> "),
        );

        println!("Downloading files for {}...", self.name);
        for path in tree.iter() {
            let target_path = base_target_path.join(path);
            if !target_path.exists() {
                let file_name = target_path.file_name().unwrap().to_string_lossy();
                file_pb.set_prefix(format!("Downloading: {}", file_name));
                file_pb.set_position(0);
                if let Some(parent) = target_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                let mut url = base_url.clone();
                {
                    let mut path_segments = match url.path_segments_mut() {
                        Ok(segments) => segments,
                        Err(_) => return Err(Error::PathSegmentsParseError),
                    };
                    path_segments.extend(
                        path.to_str()
                            .ok_or_else(|| Error::PathSegmentsParseError)?
                            .split('/'),
                    );
                }
                let mut retries = 10;
                loop {
                    match Self::download_file(&url, &target_path, &file_pb).await {
                        Ok(_) => {
                            file_pb.finish_with_message("Download completed");
                            break;
                        }
                        Err(e) => {
                            retries -= 1;
                            if retries == 0 {
                                file_pb.abandon_with_message("Download failed after 10 retries");
                                return Err(e.into());
                            }
                            file_pb
                                .abandon_with_message("Download failed. Will retry in 5 seconds.");
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
            total_pb.inc(1);
        }

        total_pb.finish_with_message("Downloaded all files in the torrent tree");
        Ok(())
    }

    pub fn check(
        &self,
        torrents_path: &PathBuf,
        target_directory: &PathBuf,
    ) -> Result<VerificationOutcome> {
        if self.file_count.is_none() {
            return Ok(VerificationOutcome::TorrentMissing);
        }

        let torrent_path = torrents_path.join(Self::get_file_name_from_url(&self.torrent_url)?);
        let torrent = Torrent::read_from_file(torrent_path)?;
        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;

        let missing_files_pb = ProgressBar::new(files.len() as u64);
        missing_files_pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")?
                .progress_chars("#>-"),
        );

        let mut missing_files = HashSet::new();
        println!("Checking for missing files...");
        for file in files.iter() {
            let path = target_directory.join(file.path.clone());
            if !path.exists() {
                missing_files.insert(path.clone());
            }
            missing_files_pb.inc(1);
        }
        missing_files_pb.finish_with_message("Completed");

        if missing_files.len() == files.len() {
            return Ok(VerificationOutcome::AllFilesMissing);
        } else if missing_files.len() > 0 {
            return Ok(VerificationOutcome::Incomplete(
                missing_files.into_iter().collect(),
                vec![],
            ));
        }

        let size_mismatch_pb = ProgressBar::new(files.len() as u64);
        size_mismatch_pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")?
                .progress_chars("#>-"),
        );
        println!("Checking for size mismatches...");
        let mut size_mismatches = Vec::new();
        for file in files.iter() {
            let path = target_directory.join(file.path.clone());
            let metadata = std::fs::metadata(&path)?;
            let size = metadata.len();
            if size != file.length as u64 {
                size_mismatches.push(path.to_string_lossy().to_string());
            }
            size_mismatch_pb.inc(1);
        }
        size_mismatch_pb.finish_with_message("Completed");

        if !size_mismatches.is_empty() {
            return Ok(VerificationOutcome::Incomplete(vec![], size_mismatches));
        }

        Ok(VerificationOutcome::Complete)
    }

    pub fn verify(
        &self,
        torrents_path: &PathBuf,
        target_directory: &PathBuf,
    ) -> Result<VerificationOutcome> {
        if self.file_count.is_none() {
            return Ok(VerificationOutcome::TorrentMissing);
        }

        let torrent_path = torrents_path.join(Self::get_file_name_from_url(&self.torrent_url)?);
        let torrent = Torrent::read_from_file(torrent_path)?;
        let piece_length = torrent.piece_length;
        let num_pieces = torrent.pieces.len();
        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;

        // If any files are missing, we can bail out before attempting to verify the release.
        let mut missing_files = HashSet::new();
        println!("Checking for missing files...");
        for file in files.iter() {
            let path = target_directory.join(file.path.clone());
            if !path.exists() {
                missing_files.insert(path.clone());
            }
        }
        if missing_files.len() == files.len() {
            return Ok(VerificationOutcome::AllFilesMissing);
        } else if missing_files.len() > 0 {
            return Ok(VerificationOutcome::Incomplete(
                missing_files.into_iter().collect(),
                vec![],
            ));
        }

        println!("All files are present. Will now attempt to verify them.");
        println!("The torrent has {} pieces to verify", num_pieces);
        let bar = ProgressBar::new(num_pieces as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")?
                .progress_chars("#>-"),
        );

        let mut mismatched_files = HashSet::new();
        let mut piece_idx = 0;
        let mut file_idx = 0;
        let mut offset: u64 = 0;

        while piece_idx < num_pieces {
            let piece_hash = &torrent.pieces[piece_idx];
            let mut buffer = Vec::new();

            // This loop reads a piece, which can span across multiple files, if the files are
            // smaller than the piece size. The 'pieces' in the torrent file are with respect to
            // the whole content rather than any individual file.
            while buffer.len() < piece_length as usize {
                let file_info = &files[file_idx];
                let file_path = target_directory.join(file_info.path.clone());

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
            if result.as_slice() != piece_hash {
                if file_idx == files.len() {
                    break;
                }
                mismatched_files.insert(files[file_idx].path.to_string_lossy().to_string());
                // If the content doesn't match at any point, the rest of the hashes won't match,
                // so we can just bail out here.
                return Ok(VerificationOutcome::Incomplete(
                    vec![],
                    mismatched_files.into_iter().collect(),
                ));
            }

            piece_idx += 1;
            bar.inc(1);
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

    async fn download_file(url: &Url, target_path: &PathBuf, file_pb: &ProgressBar) -> Result<()> {
        let client = reqwest::Client::new();
        let mut request_builder = client.get(url.clone());
        let tmp_path = target_path.with_extension("part");

        let mut start = 0;
        if tmp_path.exists() {
            start = tokio::fs::metadata(&tmp_path).await?.len() as usize;
            file_pb.set_position(start as u64);
            request_builder = request_builder.header("Range", format!("bytes={}-", start));
        }

        let mut response = request_builder.send().await?;
        if let Some(len) = response.content_length() {
            file_pb.set_length(len);
        }
        let file = if start > 0 {
            OpenOptions::new().append(true).open(&tmp_path).await?
        } else {
            tokio::fs::File::create(&tmp_path).await?
        };

        let mut writer = BufWriter::new(file);
        while let Some(chunk) = response.chunk().await? {
            writer.write_all(&chunk).await?;
            file_pb.inc(chunk.len() as u64);
        }

        writer.flush().await?;
        tokio::fs::rename(&tmp_path, target_path).await?;

        Ok(())
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
