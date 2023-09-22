use std::{collections::HashSet, fmt::{Display, format, Debug}, vec};
use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::node;
use tokio::sync::mpsc;
use url::Url;

use crate::url_helpers::parse_base_url;

// TODO: Change to use &str where possible
/// Represents a node in the MPSC queue: a search path
struct Path {
    depth:usize,
    path_array: Vec<String>, // titles so far; TODO: modify to support grep on contents (need to store HTML content strings or objects)
    path_vis: HashSet<String>, // for now, store full_url. TODO: how else can I use this with full/rel?
    relative_url:String, // about.html or what-the-interns-have-wrought-2023
    base_url:Url // https://localhost:8000/ or https://blog.janestreet.com/
}

impl Path {
    /// Create new path from given URL
    pub fn new(url:&str)->Result<Path> {
        let parsed = parse_base_url(url)?;

        Ok(Path {
            depth:0,
            path_array:vec![],
            path_vis:HashSet::new(),
            relative_url:parsed.relative,
            base_url: parsed.base
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
    
        write!(f, "(d: {}, path: {}, vis:{}, rel: {}, base: {})", self.depth, joined_arr, set_str, self.relative_url, self.base_url.to_string())
    }
}

// Improvements from Sep 15
pub async fn search2(url:&str, pattern:&str, depth_limit:usize)->Result<()> {
    let initial_path = Path::new(url)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Path>();
    println!("Starting search with: {}\n", initial_path);

    // for initial MPSC send
    let first_tx = tx.clone();

    // send first path (task)
    tokio::spawn(async move {
        first_tx.send(initial_path);
    });

    while let Some(path) = rx.recv().await {
        // parent depth, stop if +1 > limit
        let copied_depth = path.depth; 
        if copied_depth + 1 > depth_limit {
            continue;
        }

        let cloned_path = path.path_array.clone();
        let cloned_vis = path.path_vis.clone();
        let cloned_base = path.base_url.clone();
        

        tokio::spawn(async move {
            // add children nodes to mpsc - spawn new tasks
           

            // just pattern match (goal test) here, then only add children to queue
       });
    }

    Ok(())
}