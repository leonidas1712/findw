use findw::{search2::search2, consts};
use anyhow::{anyhow, Result};

// cargo r -- http://localhost:8000/index.html title 0

// TODO: replace with clap once args get more complex
#[derive(Debug)]
struct CliArgs {
    pub url:String,
    pub pattern:String,
    pub depth_limit:usize
}

/// Assumption: length = 3
fn parse_args(args:Vec<String>)->Result<CliArgs> {
    let url = args.get(1).unwrap();
    let pattern = args.get(2).unwrap();
    let depth_limit_string = args.get(3).unwrap();
    let depth_limit = depth_limit_string.parse::<usize>();

    match depth_limit {
        Ok(depth) => Ok(CliArgs { url: url.to_string(), pattern: pattern.to_string(), depth_limit:depth }),
        Err(_) => Err(anyhow!("Could not parse '{}' as a depth limit - provide a valid integer.", depth_limit_string))
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err(anyhow!(consts::USAGE))
    }

    let args = parse_args(args)?;
    search2(&args.url,args.pattern, args.depth_limit).await
}

