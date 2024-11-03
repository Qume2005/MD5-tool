use std::path::PathBuf;

use anyhow::Result;

pub async fn calculate_md5(data: Vec<u8>) -> Result<String> {
    Ok(format!("{:?}", md5::compute(data)))
}

pub async fn select_file() -> Result<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|file_handle| file_handle.path().to_path_buf())
        .ok_or(anyhow::Error::msg("Failed to open the file."))
}