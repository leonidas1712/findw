pub mod search;



pub type DynResult<T> =  Result<T, Box<dyn std::error::Error>>;

pub async fn req() -> DynResult<()> {
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