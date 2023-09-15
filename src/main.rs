use findw::{req, channel};
use findw::search::{get_links, LinkNode, expand_url};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("main");

    // expand_url("/wiki/Hoplite");
    let x = expand_url("https://developer.mozilla.org/en-US/docs/Web/CSS/:not");
    println!("css:{x}");
    
    // let doc = scraper::Html::parse_document("hi");
    // doc.tree.nodes().for_each(|x| println!("{:?}", x));

    // req().await;
    // get_links();
    // channel().await;

    // let url = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/"; - idx 51

    let x = expand_url("/wiki/Hoplite");
    println!("Leo:{}", x);

    let url = "https://en.wikipedia.org/wiki/Leonidas_I";
    let node = LinkNode::linknode_from_url(url).await;
    // println!("{}", node.unwrap());

    Ok(())
}

