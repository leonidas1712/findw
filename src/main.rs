use findw::{AsyncResult, concurrent};


#[tokio::main]
async fn main() -> AsyncResult {
    findw::hi();
    concurrent().await;

    Ok(())
}

