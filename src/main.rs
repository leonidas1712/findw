use findw::{AsyncResult, concurrent};


#[tokio::main]
async fn main() -> AsyncResult {
    findw::hi();
    findw::search::search();
    concurrent().await;

    Ok(())
}

