use serde::{Deserialize, Serialize};
use crate::api::OBSApi;

#[derive(Debug,Serialize,Deserialize)]
pub struct People {
    #[serde(rename="entry")]
    pub people: Vec<Name>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Name {
    pub name: String,
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Person {
    pub login: String,
    pub email: String,
    pub realname: String,
    pub state: String,
    pub watchlist: Watchlist
}

impl AsRef<str> for Person {
    fn as_ref(&self) -> &str {
        &self.login
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Watchlist {
    #[serde(default)]
    pub project: Vec<Name>,
    #[serde(default)]
    pub package: Vec<Name>,
    #[serde(default)]
    pub request: Vec<Name>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Tokens {
    pub count: u32,
    #[serde(default,rename="entry")]
    pub tokens: Vec<Token>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Token {
    pub id: u32,
    pub string: String,
    pub kind: String,
    pub description: String,
    pub triggered_at: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct TokenCreationResponse {
    pub code: String,
    pub summary: String,
    #[serde(rename="data")]
    pub extra: Vec<TokenCreationResponseData>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct TokenCreationResponseData {
    pub name: String,
    #[serde(rename="$value")]
    pub value: String,
}

impl OBSApi {
    pub async fn people(&self) -> reqwest::Result<People> {
        self.xml_get_with_auth("/person")
            .await
    }

    pub async fn person<T: AsRef<str>>(&self, username: T) -> reqwest::Result<Person> {
        self.xml_get_with_auth(format!("/person/{}", username.as_ref()))
            .await
    }

    pub async fn tokens<T: AsRef<str>>(&self, username: T) -> reqwest::Result<Tokens> {
        self.xml_get_with_auth(format!("/person/{}/token", username.as_ref()))
            .await
    }

    /// Create a new authentication token for a person.
    ///
    /// The token may be limited to a specific package. In this case the query parameters project and package should be provided.
    ///
    /// With an empty request body, a token of the default kind 'runservice' is created.
    pub async fn create_token<T: AsRef<str>>(&self,
                                             username: T,
                                             token_name: Option<T>,
                                             description: Option<T>,
                                             project: Option<T>,
                                             package: Option<T>,
                                             operation: Option<T>,
                                             scm_token: Option<T>) -> reqwest::Result<TokenCreationResponse> {
        let params = [
            ("token_name", token_name),
            ("description", description),
            ("project", project),
            ("package", package),
            ("operation", operation),
            ("scm_token", scm_token)
        ].into_iter()
            .filter_map(|(name, value)| {
                match value {
                    None => None,
                    Some(value) => Some(format!("{}={}", name, value.as_ref())),
                }
            })
            .collect::<Vec<_>>();
        let url_params = params.join("&");
        self.xml_post_with_auth(format!("/person/{}/token?{}", username.as_ref(), url_params))
            .await
    }

    pub async fn delete_token<T: AsRef<str>>(&self, username: T, id: u32) -> reqwest::Result<String> {
        self.delete_with_auth(format!("/person/{}/token/{}", username.as_ref(), id))
            .await
    }
}