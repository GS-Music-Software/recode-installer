use std::path::{Path, PathBuf};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use crate::consts;

pub async fn fetch(url: &str, dest: &Path) -> Result<PathBuf, String> {
    let client = reqwest::Client::builder()
        .user_agent(consts::USER_AGENT)
        .build()
        .map_err(|e| format!("http: {e}"))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("download: {e}"))?;

    if !res.status().is_success() {
        return Err(format!("download: {}", res.status()));
    }

    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| format!("create: {e}"))?;

    let mut stream = res.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("read: {e}"))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("write: {e}"))?;
    }

    file.flush().await.map_err(|e| format!("flush: {e}"))?;
    Ok(dest.to_path_buf())
}
