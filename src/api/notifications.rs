use serde::{Deserialize, Serialize};
use crate::api::OBSApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub count: u32,
    //untested
    #[serde(default)]
    pub total_pages: u32,
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub notifications: Vec<Notification>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    //untested
    pub id: u32,
    pub title: String,
    pub who: String,
    pub state: String,
    pub event_type: String,
    pub when: String,
}

impl OBSApi {
    pub async fn my_notifications(&self) -> reqwest::Result<Notifications>{
        self.xml_get_with_auth("/my/notifications")
            .await
    }
}