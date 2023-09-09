use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};
use sept11_datasets::db::*;
use sept11_datasets::Release;
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
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();
    match opt.command {
        Some(Commands::Init { torrents_path }) => {
            println!("Building release database...");
            let releases = Release::init_releases(torrents_path)?;
            for release in releases.iter() {
                println!("{release}");
            }
            println!("Saving releases to new database...");
            let db_path = get_data_directory()?.join("releases.db");
            let conn = get_db_connection(&db_path)?;
            create_db_schema(&conn)?;
            for release in releases.iter() {
                save_release(&conn, &release)?;
            }
            let _ = conn.close();
            println!("Done");
            Ok(())
        }
        None => Ok(()),
    }
}

fn get_data_directory() -> Result<PathBuf> {
    let path = dirs_next::data_dir()
        .ok_or_else(|| eyre!("Could not retrieve data directory"))?
        .join("sept11-datasets");
    if !path.exists() {
        std::fs::create_dir_all(path.clone())?;
    }
    Ok(path)
}
