use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let host = 1;
    let request_url = format!("http://192.168.1.{}", host);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;
    println!("The respopnse is {}",response.status());
    Ok(())
}
