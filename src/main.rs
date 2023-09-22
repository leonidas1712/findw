use findw::search2::search2;

const URL:&'static str = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
const LOCAL_URL:&'static str = "http://localhost:8000/index.html";
const BAD_URL:&'static str = "badurl";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    search2(BAD_URL,"title",1).await
}

