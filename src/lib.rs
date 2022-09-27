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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn main() -> reqwest::Result<()> {
        let a = obsapi_from_env().about().await?;
        println!("{:#?}", a);
        Ok(())
    }

    #[tokio::test]
    async fn about() -> reqwest::Result<()> {
        let _about = obsapi_from_env().about().await?;
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
}
