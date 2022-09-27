mod api;
pub use api::OBSApi;

#[cfg(test)]
mod tests {
    use super::*;

    fn obsapi_from_env() -> OBSApi {
        use std::env;
        OBSApi::new(
            None,
            env::var("OBS_USERNAME").ok(),
            env::var("OBS_PASSWORD").ok(),
            None
        )
    }

    #[tokio::test]
    async fn main() -> reqwest::Result<()> {
        let about = obsapi_from_env().about().await?;
        // println!("{:#?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn about() -> reqwest::Result<()> {
        let _about = obsapi_from_env().about().await?;
        // println!("{:?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn architectures() -> reqwest::Result<()> {
        let _architectures = obsapi_from_env().architectures().await?;
        // println!("{:?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn architecture() -> reqwest::Result<()> {
        let _architecture = obsapi_from_env().architecture("x86_64").await?;
        // println!("{:?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn service() -> reqwest::Result<()> {
        let _service = obsapi_from_env().service().await?;
        // println!("{:?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> reqwest::Result<()> {
        let obs = obsapi_from_env();
        let create_token = obs.create_token(
            obs.username.as_str(),
            Some("rust_test_obs_api"),
            Some("Rust test obs api"),
            None,
            None,
            None,
            None,
        ).await.unwrap();
        let created_token_id = create_token.extra
            .iter()
            .find(|d| d.name == "id")
            .map(|d| d.value.clone())
            .unwrap()
            .parse::<u32>()
            .unwrap();
        // println!("{:#?}", create_token);
        let tokens = obs.tokens(&obs.username).await.unwrap();
        // println!("{:#?}", tokens);
        let r = obs.delete_token(&obs.username, created_token_id).await.unwrap();
        // println!("{}", r);
        let tokens = obs.tokens(&obs.username).await.unwrap();
        // println!("{:#?}", tokens);
        Ok(())
    }
}
