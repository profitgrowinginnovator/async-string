use reqwest::Client;
use futures::executor::block_on;



async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {

    let client = Client::new(); 

    let response = client.get(url).send().await?;

    let text = response.text().await?;

    Ok(text)

}

fn main() {
    block_on(async_main());
}

async fn async_main() {
    let txt = fetch_data("https://httpbin.org/anything").await;
    println!("Text: {:?}", txt);
}