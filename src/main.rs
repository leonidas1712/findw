use clap::Parser;
use findw::search::search;

// Clap
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// URL to start searching from
    pub url: String,

    /// Pattern to search for
    pub pattern: String,

    /// Depth limit. Should be an integer > 0
    pub depth_limit: usize,

    /// If present, prints titles for each path instead, indicating empty titles where there are none.
    #[arg(short)]
    pub title: bool,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    search(&args.url, args.pattern, args.depth_limit, args.title).await?;

    // naive_serial_search(&args.url,args.pattern, args.depth_limit).await?;

    Ok(())
}
