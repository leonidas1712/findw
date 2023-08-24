#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.google.com/";

    let resp = reqwest::get(url)
        .await?
        .text()
        .await;

    match resp {
        Ok(res) => {
            println!("Response received");
            println!("-----------------");
            println!("{res}");
        },
        Err(err) => {
            println!("Error requesting:{}", err);
        }
    }
    Ok(())
}