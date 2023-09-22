use findw::search2::search2;
use anyhow::{anyhow, Result};

const URL:&'static str = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
const LOCAL_URL:&'static str = "http://localhost:8000/index.html";
const BAD_URL:&'static str = "badurl";
const USAGE:&'static str = "usage - findw URL PATTERN DEPTHLIMIT";

// cargo r -- http://localhost:8000/index.html title 0

// TODO: replace with clap once args get more complex
#[derive(Debug)]
struct CliArgs {
    url:String,
    pattern:String,
    depth_limit:usize
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
        return Err(anyhow!(USAGE))
    }

    let args = parse_args(args)?;
    println!("{:?}", args);

    search2(LOCAL_URL,"title",1).await
}

