use reqwest::header;
use serde_xml_rs::from_str;
// use serde_xml_rs::to_string;
mod general;

pub struct OBSApi {
    endpoint: String,
    username: String,
    password: String,
    token: String,
}

impl Default for OBSApi {
    fn default() -> Self {
        Self {
            endpoint: "https://api.opensuse.org".to_string(),
            username: String::default(),
            password: String::default(),
            token: String::default(),
        }
    }
}

impl OBSApi {
    pub fn new<T: ToString>(endpoint: Option<T>, login_username: Option<T>, login_password: Option<T>, token: Option<T>) -> Self {
        Self {
            endpoint: endpoint.map(|s| s.to_string()).unwrap_or(OBSApi::default().endpoint),
            username: login_username.map(|s| s.to_string()).unwrap_or_default(),
            password: login_password.map(|s| s.to_string()).unwrap_or_default(),
            token: token.map(|s| s.to_string()).unwrap_or_default(),
        }
    }
    pub fn new_with_login<T: ToString>(username: T, password: T, endpoint: T) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            ..OBSApi::default()
        }
    }

    pub fn add_credentials<T: ToString>(&mut self, username: T, password: T) {
        self.username = username.to_string();
        self.password = password.to_string();
    }

    pub fn add_token<T: ToString>(&mut self, token: T) {
        self.token = token.to_string();
    }

    pub async fn get<S: AsRef<str>>(&self, path: S) -> reqwest::Result<String> {
        reqwest::get(format!("{}{}", self.endpoint, path.as_ref()))
            .await?
            .error_for_status()?
            .text()
            .await
    }

    pub async fn xml_get<'a, T: serde::Deserialize<'a>, S: AsRef<str>>(&self, path: S) -> reqwest::Result<T> {
        let resp = self.get(path).await?;
        // println!("{}", resp);
        let resp_structure: T = from_str(&resp).unwrap();
        Ok(resp_structure)
    }

    pub async fn get_with_auth<S: AsRef<str>>(&self, path: S) -> reqwest::Result<String> {
        reqwest::Client::builder()
            .build()?
            .get(format!("{}{}", self.endpoint, path.as_ref()))
            .basic_auth(&self.username, Some(&self.password))
            .header(header::ACCEPT, "application/xml; charset=utf-8")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    pub async fn xml_get_with_auth<'a, T: serde::Deserialize<'a>, S: AsRef<str>>(&self, path: S) -> reqwest::Result<T> {
        let resp = self.get_with_auth(path).await?;
        // println!("{}", resp);
        let resp_structure: T = from_str(&resp).unwrap();
        Ok(resp_structure)
    }

    pub async fn post_with_auth<S: AsRef<str>>(&self, path: S) -> reqwest::Result<String> {
        reqwest::Client::builder()
            .build()?
            .post(format!("{}{}", self.endpoint, path.as_ref()))
            .basic_auth(&self.username, Some(&self.password))
            .header(header::ACCEPT, "application/xml; charset=utf-8")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    pub async fn xml_post_with_auth<'a, T: serde::Deserialize<'a>, S: AsRef<str>>(&self, path: S) -> reqwest::Result<T> {
        let resp = self.post_with_auth(path).await?;
        // println!("{}", resp);
        let resp_structure: T = from_str(&resp).unwrap();
        Ok(resp_structure)
    }

    pub async fn post_with_token<S: AsRef<str>>(&self, path: S) -> reqwest::Result<String> {
        reqwest::Client::builder()
            .build()?
            .post(format!("{}{}", self.endpoint, path.as_ref()))
            .header(header::AUTHORIZATION, self.token_header())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    pub async fn xml_post_with_token<'a, T: serde::Deserialize<'a>, S: AsRef<str>>(&self, path: S) -> reqwest::Result<T> {
        let resp = self.post_with_token(path).await?;
        let resp_structure: T = from_str(&resp).unwrap();
        Ok(resp_structure)
    }

    fn token_header(&self) -> String {
        format!("Token {}", self.token)
    }
}