use findw::{DynResult, concurrent};
use findw::search::get_links;


#[tokio::main]
async fn main() -> DynResult<()> {
    get_links();

    Ok(())
}

