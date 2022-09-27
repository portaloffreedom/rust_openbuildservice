mod api;
pub use api::OBSApi;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn main() -> reqwest::Result<()> {
        let a = OBSApi::default().architecture("x86_64").await?;
        println!("{:#?}", a);
        Ok(())
    }

    #[tokio::test]
    async fn about() -> reqwest::Result<()> {
        let _about = OBSApi::default().about().await?;
        Ok(())
    }

    #[tokio::test]
    async fn architectures() -> reqwest::Result<()> {
        let _architectures = OBSApi::default().architectures().await?;
        // println!("{:?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn architecture() -> reqwest::Result<()> {
        let _architecture = OBSApi::default().architecture("x86_64").await?;
        // println!("{:?}", about);
        Ok(())
    }

    #[tokio::test]
    async fn service() -> reqwest::Result<()> {
        let _service = OBSApi::default().service().await?;
        // println!("{:?}", about);
        Ok(())
    }
}
