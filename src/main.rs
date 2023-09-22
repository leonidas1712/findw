use findw::{req, channel};
use findw::search::{get_links, LinkNode, expand_url, search};
use url::{Url};
use findw::url_helpers::debug_url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("MAIN");
    let URL = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
    let LOCAL_URL = "http://localhost:8000/index.html";

    debug_url(LOCAL_URL);

    // search(URL,"Jane",2).await?;

    Ok(())
}

