use findw::{req, channel};
use findw::search::{get_links, LinkNode, expand_url, search};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("main");

    // let url = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/"; - idx 51 has full URL


    // let URL = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";

    // let node = LinkNode::linknode_from_url(URL).await;
    // println!("{}", node.unwrap());

    // let URL = "https://en.wikipedia.org/wiki/Leonidas_I";
     let URL = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
    search(URL,"Jane",2).await?;

    Ok(())
}

