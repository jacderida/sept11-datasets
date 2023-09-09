use crate::{Release, VerificationOutcome};
use rusqlite::{Connection, Result};
use std::path::Path;

pub fn get_db_connection<P: AsRef<Path>>(path: P) -> Result<Connection> {
    Connection::open(path)
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
