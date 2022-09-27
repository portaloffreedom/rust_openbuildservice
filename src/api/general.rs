use serde::{Deserialize, Serialize};
use crate::api::OBSApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct About {
    title: String,
    description: String,
    revision: String,
    last_deployment: String,
    commit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Architectures {
    #[serde(rename="entry")]
    entries: Vec<Architecture>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Architecture {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceList {
    service: Vec<Service>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    name: String,
    summary: String,
    description: String,
    #[serde(default, rename="parameter")]
    parameters: Vec<ServiceParam>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceParam {
    name: String,
    description: String,
    #[serde(default)]
    required: Option<()>,
    #[serde(default, rename="allowedvalue")]
    allowed_values: Vec<String>,
}

impl OBSApi {
    pub async fn about(&self) -> reqwest::Result<About>{
        self.xml_get("/about")
            .await
    }

    pub async fn architectures(&self) -> reqwest::Result<Architectures>{
        self.xml_get_with_auth("/architectures")
            .await
    }

    pub async fn architecture<T: AsRef<str>>(&self, architecture: T) -> reqwest::Result<Architecture>{
        self.xml_get_with_auth(format!("/architectures/{}", architecture.as_ref()))
            .await
    }

    pub async fn service(&self) -> reqwest::Result<ServiceList>{
        self.xml_get_with_auth("/service")
            .await
    }
}