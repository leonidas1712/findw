use findw::search2::search2;
use findw::url_helpers::debug_url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("MAIN");
    let URL = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
    let LOCAL_URL = "http://localhost:8000/index.html";

    debug_url(LOCAL_URL);

    search2(LOCAL_URL,"title",1).await?;

    Ok(())
}

