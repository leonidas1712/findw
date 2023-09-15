use findw::{req, channel};
use findw::search::{get_links, LinkNode};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("main");
    // req().await;
    // get_links();
    // channel().await;

    // let node = LinkNode::linknode_from_url("url").await;
    // println!("{}", node);

    Ok(())
}

