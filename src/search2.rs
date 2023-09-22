use std::{collections::HashSet, fmt::{Display, format, Debug}};
use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::node;
use tokio::sync::mpsc;

// TODO: Change to use &str where possible
/// Represents a node in the MPSC queue: a search path
struct Path {
    path_array: Vec<String>,
    path_vis: HashSet<String>,
    relative_url:String, // about.html or what-the-interns-have-wrought-2023
    base_url:String // https://localhost:8000/ or https://blog.janestreet.com/
}