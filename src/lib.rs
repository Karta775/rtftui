use directories::BaseDirs;
use log::trace;
use reqwest;
use std::io::Cursor;

const DEVDOCS_GIT: &str = "https://github.com/freeCodeCamp/devdocs/archive/refs/heads/main.zip";

/// Sync devdocs source code and extract to local data directory
pub async fn sync_repo() -> Result<(), Box<dyn std::error::Error>> {
    trace!("Starting devdoc sync");

    // Get the project directories
    let dirs = BaseDirs::new().unwrap();
    let local_storage = dirs.data_local_dir().join("rtftui");

    // Setup zip archive
    let source_zip = reqwest::get(DEVDOCS_GIT).await?.bytes().await?;
    let reader = Cursor::new(source_zip);
    let mut archive = zip::ZipArchive::new(reader)?;

    // Extract the zip file to the data directory
    std::fs::create_dir_all(&local_storage)?;
    archive.extract(&local_storage)?;

    Ok(())
}
