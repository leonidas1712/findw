use findw::{search2::search2, search_without_stop::search_without_stop};
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

    /// If present, runs hanging version (doesn't stop)
    #[arg(short)]
    pub no_stop:bool
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    if args.no_stop {
        search_without_stop(&args.url, args.pattern, args.depth_limit).await
    } else {
        search2(&args.url,args.pattern, args.depth_limit).await
    }
}

