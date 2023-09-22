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
    path_array: Vec<String>,
    path_vis: HashSet<String>,
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