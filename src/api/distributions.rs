use serde::{Deserialize, Serialize};
use crate::api::OBSApi;


#[derive(Debug, Serialize, Deserialize)]
pub struct Distributions {
    #[serde(rename="distribution")]
    pub distributions: Vec<Distribution>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub vendor: String,
    pub version: String,
    pub id: u32,
    pub name: String,
    pub project: String,
    pub reponame: String,
    pub repository: String,
    pub link: String,
    #[serde(default, rename="icon")]
    pub icons: Vec<Icon>,
    #[serde(default, rename="architecture")]
    pub architectures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

impl OBSApi {
    pub async fn distributions(&self) -> reqwest::Result<Distributions> {
        self.xml_get_with_auth("/distributions")
            .await
    }

    pub async fn distributions_including_remotes(&self) -> reqwest::Result<Distributions> {
        self.xml_get_with_auth("/distributions/include_remotes")
            .await
    }

    pub async fn distribution(&self, distribution: u32) -> reqwest::Result<Distribution> {
        self.xml_get_with_auth(format!("/distributions/{}", distribution))
            .await
    }
}