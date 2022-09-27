use serde::{Deserialize, Serialize};
use crate::api::OBSApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    #[serde(rename="entry")]
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
}

impl AsRef<str> for Project {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repos {
    #[serde(rename="entry")]
    pub repos: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
}

impl AsRef<str> for Repo {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum RepoCommand {
    wipe,
    restartbuild,
    killbuild,
    abortbuild,
    rebuild,
    unpublish,
    sendsysrq
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoResults {
    #[serde(rename="result")]
    pub results: Vec<RepoResult>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Code {
    unresolvable,
    succeeded,
    failed,
    published,
    blocked,
    excluded,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoResult {
    pub project: String,
    pub repository: String,
    pub arch: String,
    pub code: Code,
    pub state: String,
    #[serde(rename="status")]
    pub packages_status: Vec<PackageStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageStatus {
    pub package: String,
    pub code: Code,
    pub details: Option<String>,
}

impl OBSApi {
    pub async fn projects(&self) -> reqwest::Result<Projects>{
        self.xml_get_with_auth("/build")
            .await
    }

    pub async fn project_repositories<T: AsRef<str>>(&self, project: T) -> reqwest::Result<Repos> {
        self.xml_get_with_auth(format!("/build/{}", project.as_ref()))
            .await
    }

    // pub async fn project_repository_command<T: AsRef<str>>(&self, project: T, command: RepoCommand) -> reqwest::Result<Repos> {
    //     todo!("this is dangerous, I will implement it later");
    //     self.xml_post_with_auth(format!("/build/{}", project.as_ref()))
    //         .await
    // }

    pub async fn project_results<T: AsRef<str>>(&self, project: T) -> reqwest::Result<RepoResults> {
        self.xml_get_with_auth(format!("/build/{}/_result", project.as_ref()))
            .await
    }
}