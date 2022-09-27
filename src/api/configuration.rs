use serde::{Deserialize, Serialize};
use crate::api::OBSApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub title: String,
    pub description: String,
    pub name: String,
    pub download_on_demand: String,
    pub enforce_project_keys: String,
    pub anonymous: String,
    pub registration: String,
    pub default_access_disabled: String,
    pub allow_user_to_create_home_project: String,
    pub disallow_group_creation: String,
    pub change_password: String,
    pub obs_url: String,
    pub api_url: String,
    pub hide_private_options: String,
    pub gravatar: String,
    pub download_url: String,
    pub ymp_url: String,
    pub bugzilla_url: String,
    pub no_proxy: String,
    pub cleanup_after_days: String,
    pub theme: String,
    pub cleanup_empty_projects: String,
    pub disable_publish_for_branches: String,
    pub admin_email: String,
    pub unlisted_projects_filter: String,
    pub unlisted_projects_filter_description: String,
    pub tos_url: String,
    pub schedulers: Schedulers,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedulers {
    pub arch: Vec<String>
}

impl OBSApi {
    pub async fn configuration(&self) -> reqwest::Result<Configuration>{
        self.xml_get_with_auth("/configuration")
            .await
    }
}