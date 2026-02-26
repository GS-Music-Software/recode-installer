use serde::Deserialize;
use crate::consts;

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn latest() -> Result<Release, String> {
    let url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        consts::REPO,
    );

    let client = reqwest::Client::builder()
        .user_agent(consts::USER_AGENT)
        .build()
        .map_err(|e| format!("http client: {e}"))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("fetch release: {e}"))?;

    if !res.status().is_success() {
        return Err(format!("github api: {}", res.status()));
    }

    res.json::<Release>()
        .await
        .map_err(|e| format!("parse release: {e}"))
}

pub fn extra_assets(release: &Release) -> Vec<&Asset> {
    release
        .assets
        .iter()
        .filter(|a| a.name.to_lowercase().ends_with(".dll"))
        .collect()
}

pub fn pick_asset(release: &Release) -> Result<&Asset, String> {
    let os = std::env::consts::OS;
    let needle = match os {
        "linux" => "linux",
        "windows" => "windows",
        _ => return Err(format!("unsupported os: {os}")),
    };

    release
        .assets
        .iter()
        .find(|a| a.name.to_lowercase().contains(needle))
        .ok_or_else(|| format!("no assets for {os} in release {}", release.tag_name))
}
