use findw::{DynResult, concurrent};


#[tokio::main]
async fn main() -> DynResult<()> {
    findw::hi();
    findw::search::search();
    concurrent().await;

    Ok(())
}

