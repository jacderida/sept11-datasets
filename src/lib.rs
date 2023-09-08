pub mod error;

use crate::error::{Error, Result};
use indicatif::{ProgressBar, ProgressStyle};
use lava_torrent::torrent::v1::Torrent;
use sha1::{Digest, Sha1};
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::PathBuf;
use url::Url;

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
    pub date: String,
    pub name: String,
    pub file_count: Option<usize>,
    pub size: Option<u64>,
    pub torrent_url: Url,
    pub torrent_path: Option<PathBuf>,
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.torrent_path.is_none() {
            write!(f, "{}: {} -- NO TORRENT", self.date, self.name)
        } else {
            write!(
                f,
                "{}: {} ({} files, {})",
                self.date,
                self.name,
                self.file_count
                    .map_or("None".to_string(), |n| n.to_string()),
                self.size
                    .map_or("None".to_string(), |s| self.bytes_to_human_readable(s))
            )
        }
    }
}

impl Release {
    pub fn init_from_table(table_path: PathBuf, torrents_path: PathBuf) -> Result<Vec<Release>> {
        let file = File::open(table_path)?;
        let reader = BufReader::new(file);
        let mut releases = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() != 3 {
                return Err(Error::MalformedReleaseTable);
            }

            let date = parts[0].trim().to_string();
            let torrent_url = parts[1].trim();
            let name = parts[2].trim().to_string();
            let url = Url::parse(torrent_url)?;
            let file_name = url
                .path_segments()
                .ok_or(Error::PathSegmentsParseError)?
                .last()
                .ok_or(Error::PathSegmentsParseError)?;

            let mut torrent_path = torrents_path.clone();
            torrent_path.push(file_name);
            let (torrent_path, file_count, size) = if torrent_path.exists() {
                match Torrent::read_from_file(torrent_path.clone()) {
                    Ok(torrent) => {
                        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;
                        let mut size = 0;
                        for file in files.iter() {
                            size += file.length;
                        }
                        (Some(torrent_path), Some(files.len()), Some(size as u64))
                    }
                    Err(_) => (None, None, None),
                }
            } else {
                (None, None, None)
            };

            releases.push(Release {
                date,
                name,
                file_count,
                size,
                torrent_url: url,
                torrent_path,
            });
        }
        Ok(releases)
    }

    pub fn verify(&self, target_directory: PathBuf) -> Result<VerificationOutcome> {
        let mut mismatched_files = HashSet::new();
        let mut missing_files = HashSet::new();

        if self.torrent_path.is_none() {
            return Ok(VerificationOutcome::TorrentMissing);
        }

        let torrent = Torrent::read_from_file(self.torrent_path.as_ref().unwrap())?;
        let piece_length = torrent.piece_length;
        let num_pieces = torrent.pieces.len();
        let files = torrent.files.ok_or_else(|| Error::TorrentFilesError)?;

        println!("Verifying release {}", self.name);
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
            if missing_files.len() == files.len() {
                return Ok(VerificationOutcome::AllFilesMissing);
            }
            return Ok(VerificationOutcome::Incomplete(
                missing_files.into_iter().collect(),
                mismatched_files.into_iter().collect(),
            ));
        }

        Ok(VerificationOutcome::Verified)
    }

    fn bytes_to_human_readable(&self, bytes: u64) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        }
    }
}
