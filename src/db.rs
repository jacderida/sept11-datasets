use crate::error::Result;
use crate::{Release, VerificationOutcome};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

pub fn get_db_connection<P: AsRef<Path>>(path: P) -> Result<Connection> {
    Ok(Connection::open(path)?)
}

pub fn create_db_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS releases (
            id TEXT PRIMARY KEY NOT NULL,
            date TEXT NOT NULL,
            name TEXT NOT NULL,
            file_count INTEGER,
            size INTEGER,
            torrent_url TEXT NOT NULL,
            verification_outcome TEXT NOT NULL
        );",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS incomplete_files (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            release_id TEXT NOT NULL,
            file_path TEXT NOT NULL,
            status TEXT NOT NULL,
            FOREIGN KEY (release_id) REFERENCES releases(id)
        );",
        [],
    )?;
    Ok(())
}

pub fn save_release(conn: &Connection, release: &Release) -> Result<()> {
    let file_count: Option<i64> = release.file_count.map(|v| v as i64);
    let size: Option<i64> = release.size.map(|v| v as i64);
    let verification_status = match &release.verification_outcome {
        Some(VerificationOutcome::Verified) => "VERIFIED".to_string(),
        Some(VerificationOutcome::TorrentMissing) => "NO TORRENT".to_string(),
        Some(VerificationOutcome::Incomplete(_, _)) => "INCOMPLETE".to_string(),
        Some(VerificationOutcome::AllFilesMissing) => "MISSING".to_string(),
        None => "UNKNOWN".to_string(),
    };
    conn.execute(
        "INSERT INTO releases (id, date, name, file_count, size, torrent_url, verification_outcome) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[
            &release.id as &dyn rusqlite::ToSql,
            &release.date,
            &release.name,
            &file_count,
            &size,
            &release.torrent_url.to_string(),
            &verification_status
        ],
    )?;

    if let Some(VerificationOutcome::Incomplete(missing, corrupted)) = &release.verification_outcome
    {
        for path in missing {
            conn.execute(
                "INSERT INTO incomplete_files (release_id, file_path, status) VALUES (?1, ?2, 'MISSING')",
                [&release.id, &path.to_string_lossy().to_string()],
            )?;
        }
        for path in corrupted {
            conn.execute(
                "INSERT INTO incomplete_files (release_id, file_path, status) VALUES (?1, ?2, 'CORRUPTED')",
                [&release.id, &path],
            )?;
        }
    }
    Ok(())
}

pub fn get_releases(conn: &Connection) -> Result<Vec<Release>> {
    let mut statement = conn.prepare(
        "SELECT id, date, name, file_count, size, torrent_url, verification_outcome FROM releases",
    )?;
    let mut rows = statement.query([])?;
    let mut releases = Vec::new();

    while let Some(row) = rows.next()? {
        let mut release = Release::from_row(row)?;
        if release.verification_outcome.is_none() {
            let mut missing_files = Vec::new();
            let mut corrupted_files = Vec::new();
            let mut files_statement = conn
                .prepare("SELECT file_path, status FROM incomplete_files WHERE release_id = ?1")?;
            let files_iter = files_statement.query_map([&release.id], |row| {
                let file_path: String = row.get(0)?;
                let status: String = row.get(1)?;
                Ok((file_path, status))
            })?;
            for file_result in files_iter {
                let (file_path, status) = file_result?;
                let path = PathBuf::from(file_path);
                if status == "MISSING" {
                    missing_files.push(path);
                } else if status == "CORRUPTED" {
                    corrupted_files.push(path.to_string_lossy().into_owned());
                }
            }
            release.verification_outcome = Some(VerificationOutcome::Incomplete(
                missing_files,
                corrupted_files,
            ));
        }
        releases.push(release);
    }
    Ok(releases)
}
