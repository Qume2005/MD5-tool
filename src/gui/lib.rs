use anyhow::Result;

pub async fn calculate_md5(data: Vec<u8>) -> Result<String> {
    Ok(format!("{:?}", md5::compute(data)))
}