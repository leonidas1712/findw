use findw::{req, channel};
use findw::search::{get_links, LinkNode, expand_url, search};
use url::{Url};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("MAIN");
    let URL = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";

    let parsed = Url::parse(URL)?;
    
    println!("Domain: {:?}", parsed.domain());
    println!("Base: {:?}", parsed.scheme());


    // search(URL,"Jane",2).await?;

    Ok(())
}

