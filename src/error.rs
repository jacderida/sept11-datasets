use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    LavaTorrentError(#[from] lava_torrent::LavaTorrentError),
    #[error("The release table has a row that is not correctly formed with 3 columns")]
    MalformedReleaseTable,
    #[error("Cannot parse path segments from torrent URL")]
    PathSegmentsParseError,
    #[error("The top level directory for the release could not be obtained")]
    ReleaseDirectoryNotObtained,
    #[error("There is no release with ID {0}")]
    ReleaseNotFound(String),
    #[error(transparent)]
    SqlError(#[from] rusqlite::Error),
    #[error(transparent)]
    TemplateError(#[from] indicatif::style::TemplateError),
    #[error("Cannot retrieve torrent files")]
    TorrentFilesError,
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("Verification report error: {0}")]
    VerificationReportError(String),
}
