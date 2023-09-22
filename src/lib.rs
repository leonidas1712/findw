pub mod search;
pub mod url_helpers;

pub async fn req() -> anyhow::Result<()> {
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



// tokio learning
use tokio::sync::mpsc;
pub async fn channel() {
    let (tx, mut rx) = mpsc::unbounded_channel::<usize>();

    tokio::spawn(async move {
        tx.send(200);
    });

    while let Some(msg) = rx.recv().await {
        println!("Msg: {}", msg);
    }
    
}