
type AsyncResult =  Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AsyncResult {
    concurrent().await;

    Ok(())
}

async fn test(n:usize) {
    println!("Test: {}", n);
}

async fn concurrent() -> AsyncResult {
    let t1 = tokio::spawn({
        test(10)
    });

    let t2 = tokio::spawn({
        test(20)
    });

    let t3 = tokio::spawn({
        test(30)
    });

    tokio::join!(t1, t2, t3);

    Ok(())
}

async fn req() -> AsyncResult {
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