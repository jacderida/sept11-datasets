use crate::error::{Error, Result};
use crate::{Release, VerificationOutcome};
use rusqlite::{params, Connection, OptionalExtension};
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
            directory TEXT,
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
    let has_notes_column: bool = conn
        .query_row("PRAGMA table_info(releases);", [], |row| {
            Ok(row.get::<_, String>(1)? == "notes")
        })
        .optional()?
        .unwrap_or(false);

    if !has_notes_column {
        conn.execute("ALTER TABLE releases ADD COLUMN notes TEXT;", [])?;
    }
    Ok(())
}

pub fn save_new_release(conn: &Connection, release: &Release) -> Result<()> {
    let file_count: Option<i64> = release.file_count.map(|v| v as i64);
    let size: Option<i64> = release.size.map(|v| v as i64);
    let verification_status = match &release.verification_outcome {
        Some(VerificationOutcome::Complete) => "COMPLETE".to_string(),
        Some(VerificationOutcome::Verified) => "VERIFIED".to_string(),
        Some(VerificationOutcome::TorrentMissing) => "NO TORRENT".to_string(),
        Some(VerificationOutcome::Incomplete(_, _)) => "INCOMPLETE".to_string(),
        Some(VerificationOutcome::AllFilesMissing) => "MISSING".to_string(),
        None => "UNKNOWN".to_string(),
    };
    conn.execute(
        "INSERT INTO releases (id, date, name, directory, file_count, size, torrent_url, verification_outcome) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[
            &release.id as &dyn rusqlite::ToSql,
            &release.date,
            &release.name,
            &release.directory,
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

pub fn save_verification_result(conn: &mut Connection, release: &Release) -> Result<()> {
    let tx = conn.transaction()?;
    let outcome = release.verification_outcome.as_ref().unwrap();
    let outcome_str = match outcome {
        VerificationOutcome::Complete => "COMPLETE",
        VerificationOutcome::Verified => "VERIFIED",
        VerificationOutcome::TorrentMissing => "NO TORRENT",
        VerificationOutcome::Incomplete(_, _) => "INCOMPLETE",
        VerificationOutcome::AllFilesMissing => "MISSING",
    };
    tx.execute(
        "UPDATE releases SET verification_outcome = ?1 WHERE id = ?2",
        params![outcome_str, release.id],
    )?;

    if let VerificationOutcome::Incomplete(missing_files, corrupted_files) = outcome {
        for missing in missing_files.iter() {
            tx.execute(
                "INSERT INTO incomplete_files (release_id, file_path, status) VALUES (?1, ?2, ?3)",
                params![release.id, missing.to_str().unwrap(), "MISSING"],
            )?;
        }
        for corrupted in corrupted_files.iter() {
            tx.execute(
                "INSERT INTO incomplete_files (release_id, file_path, status) VALUES (?1, ?2, ?3)",
                params![release.id, corrupted, "CORRUPTED"],
            )?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn save_notes(conn: &Connection, release_id: &str, notes: &str) -> Result<()> {
    conn.execute(
        "UPDATE releases SET notes = ?1 WHERE id = ?2",
        params![notes, release_id],
    )?;
    Ok(())
}

pub fn reset_verification_result(conn: &mut Connection, release_id: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE releases SET verification_outcome = 'UNKNOWN' WHERE id = ?1",
        params![release_id],
    )?;
    tx.execute(
        "DELETE FROM incomplete_files WHERE release_id = ?1",
        params![release_id],
    )?;
    tx.commit()?;
    Ok(())
}

pub fn get_releases(conn: &Connection) -> Result<Vec<Release>> {
    let mut statement = conn.prepare(
        "SELECT id, date, name, \
            directory, file_count, size, torrent_url, verification_outcome, notes FROM releases",
    )?;
    let mut rows = statement.query([])?;
    let mut releases = Vec::new();

    while let Some(row) = rows.next()? {
        let release_id: String = row.get(0)?;
        let (missing_files, corrupted_files) =
            get_incomplete_verification_data(&conn, &release_id)?;
        let release = Release::from_row(row, &missing_files, &corrupted_files)?;
        releases.push(release);
    }
    Ok(releases)
}

pub fn get_missing_releases(conn: &Connection) -> Result<Vec<Release>> {
    let mut statement = conn.prepare(
        "SELECT id, date, name, directory, file_count, size, torrent_url, verification_outcome, notes FROM \
            releases WHERE verification_outcome = 'MISSING'",
    )?;
    let mut rows = statement.query([])?;
    let mut releases = Vec::new();

    while let Some(row) = rows.next()? {
        let release_id: String = row.get(0)?;
        let (missing_files, corrupted_files) =
            get_incomplete_verification_data(&conn, &release_id)?;
        let release = Release::from_row(row, &missing_files, &corrupted_files)?;
        releases.push(release);
    }
    Ok(releases)
}

pub fn get_release_by_id(conn: &Connection, release_id: &str) -> Result<Release> {
    let mut statement = conn.prepare(
        "SELECT \
        id, date, name, \
        directory, file_count, size, \
        torrent_url, verification_outcome, notes FROM releases WHERE id = ?1",
    )?;
    let mut rows = statement.query(params![release_id])?;

    if let Some(row) = rows.next()? {
        let (missing_files, corrupted_files) =
            get_incomplete_verification_data(&conn, &release_id)?;
        let release = Release::from_row(&row, &missing_files, &corrupted_files)?;
        return Ok(release);
    }
    Err(Error::ReleaseNotFound(release_id.to_string()))
}

fn get_incomplete_verification_data(
    conn: &Connection,
    release_id: &str,
) -> Result<(Vec<PathBuf>, Vec<String>)> {
    // If the verification result was anything other than INCOMPLETE, the returned lists will be
    // empty.
    let mut missing_files = Vec::new();
    let mut corrupted_files = Vec::new();
    let mut files_statement =
        conn.prepare("SELECT file_path, status FROM incomplete_files WHERE release_id = ?1")?;
    let files_iter = files_statement.query_map([&release_id], |row| {
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
    Ok((missing_files.clone(), corrupted_files.clone()))
}
