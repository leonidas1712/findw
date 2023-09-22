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
    fn new(url:&str)->Result<Path> {
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
pub async fn search2(url:&str, pattern:&str, limit:usize)->Result<()> {
    let initial_path = Path::new(url)?;
    // println!("Initial:{}", initial_path.to_string());

    
    Ok(())
}