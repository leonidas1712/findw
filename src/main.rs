use findw::{search::search, search_without_stop::search_without_stop, url_helpers::parse_base_url};
use clap::Parser;

// Clap
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// URL to start searching from
    pub url: String,

    /// Pattern to search for
    pub pattern:String,

    /// Depth limit. Should be an integer > 0
    pub depth_limit:usize,

    /// If present, prints titles for each path instead, indicating empty titles where there are none.
    #[arg(short)]
    pub title: bool
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    search(&args.url,args.pattern, args.depth_limit, args.title).await?;
    Ok(())
}