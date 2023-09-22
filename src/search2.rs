use std::{collections::HashSet, fmt::{Display, format, Debug}, vec, process::exit};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::get;
use scraper::node;
use tokio::sync::mpsc;
use url::Url;

use crate::url_helpers::{parse_base_url, ParsedUrl};

// TODO: Change to use &str where possible
/// Represents a node in the MPSC queue: a search path
#[derive(Clone)]
struct Path {
    depth:usize,
    path_array: Vec<String>, // titles so far; TODO: modify to support grep on contents (need to store HTML content strings or objects)
    path_vis: HashSet<String>, // for now, store full_url. TODO: how else can I use this with full/rel?
    parsed_url:ParsedUrl // wraps around base Url and relative url String
}

impl Path {
    /// Create new path from given URL
    pub fn new(url:&str)->Result<Path> {
        let parsed_url = parse_base_url(url)?;

        Ok(Path {
            depth:0,
            path_array:vec![],
            path_vis:HashSet::new(),
            parsed_url
        })
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined_arr:String = self.path_array.iter().map(|s| s.to_string()).collect();
        let set_str:String = self.path_vis.iter().map(|s| s.to_string()).collect();

        let joined_arr = if joined_arr.len() == 0 {
            String::from("[]")
        } else {
            joined_arr
        };

        let set_str = if set_str.len() == 0 {
            String::from("{}")
        } else {
            set_str
        };
    
        write!(f, "(d: {}, path: {}, vis:{}, rel: {}, base: {})", self.depth, joined_arr, set_str, self.parsed_url.relative, self.parsed_url.base.to_string())
    }
}

// get_info(full_url:&str) -> (child_hrefs:Vec<String>, page_title:Option<String>)
    // 1. get req for full_url
    // 2. parse HTML, get title and hrefs
    // 3. return out


// Improvements from Sep 15
// Program stops when all tx go out of scope
    // Eventually children are no longer added so no more txs to clone - all dropped
pub async fn search2(url:&str, pattern:&str, depth_limit:usize)->Result<()> {
    let initial_path = Path::new(url)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Path>();
    println!("Starting search with: {}\n", initial_path);

    // for initial MPSC send - need other tx to clone for remaining workers
    let first_tx = tx.clone();

    // send first path (task)
    tokio::spawn(async move {
        first_tx.send(initial_path);
    });

    while let Some(path) = rx.recv().await {
        println!("Full URL in main:{}", &path.parsed_url.get_full_url());

        // parent depth, stop if +1 > limit
        let copied_depth = path.depth;

        

        // cloned path for new task - Strat 1
        let cloned_path = path.clone();


        tokio::spawn(async move {
            // BLOCKING WITHIN TASK: add children nodes to mpsc - spawn new tasks
                // TODO: handle task failure properly
            println!("SPAWN");
            let get_info = cloned_path.parsed_url.get_info().await;
            
            match get_info {
                Ok(info) => {
                    println!("INFO: {}", info);

                    // reached limit
                    if copied_depth + 1 > depth_limit  {
                        exit(0);
                    }

                },
                Err(err) => {
                    println!("ERROR: {}", err);
                }
            }

            // just pattern match (goal test) here, then only add children to queue
       });
    }

    Ok(())
}