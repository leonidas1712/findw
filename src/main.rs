use findw::{req, channel};
use findw::search::{get_links, LinkNode};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("main");

    
    // let doc = scraper::Html::parse_document("hi");
    // doc.tree.nodes().for_each(|x| println!("{:?}", x));

    // req().await;
    // get_links();
    // channel().await;

    // let url = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/"; - idx 51
    
    let url = "https://en.wikipedia.org/wiki/Leonidas_I";
    let node = LinkNode::linknode_from_url(url).await;
    // println!("{}", node.unwrap());

    Ok(())
}

