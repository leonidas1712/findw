use std::{collections::HashSet, fmt::{Display, format, Debug}, vec, process::exit};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::get;
use scraper::node;
use tokio::sync::mpsc;
use url::Url;

use crate::url_helpers::{parse_base_url, ParsedUrl, is_relative};

// TODO: Change to use &str where possible
/// Represents a node in the MPSC queue: a search path
#[derive(Clone)]
struct Path {
    pub depth:usize,
    pub path_array: Vec<String>, // titles so far; TODO: modify to support grep on contents (need to store HTML content strings or objects)
    pub path_vis: HashSet<String>, // for now, store full_url. TODO: how else can I use this with full/rel?
    pub parsed_url:ParsedUrl // wraps around base Url and relative url String
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

    pub fn new_from_data(depth:usize, path_array:Vec<String> , path_vis: HashSet<String>, parsed_url:ParsedUrl)->Path {
        Path {
            depth,
            path_array,
            path_vis,
            parsed_url
        }
    }

    // join current path titles with newest
        // because we only get newest upon get req
    pub fn print_path(&self, latest_title:&str) -> String {
        let joined = self.path_array.join(" => ");
        joined + latest_title
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
pub async fn search2(url:&str, pattern:String, depth_limit:usize)->Result<()> {
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
        // let mut cloned_path = path.clone();
        let cloned_tx = tx.clone();
        let cloned_pat = pattern.clone();


        tokio::spawn(async move {
            // BLOCKING WITHIN TASK: add children nodes to mpsc - spawn new tasks
                // TODO: handle task failure properly
            let get_info = path.parsed_url.get_info().await;
            
            match get_info {
                Ok(info) => {
                    // println!("INFO: {}", info);


                    // get title, hrefs
                    let page_title = info.page_title;
                    let child_hrefs = info.child_hrefs;


                    // GOAL TEST AND PRINT: right now just title.contains

                    // map to (title, bool) where bool=T means title meets goal test
                    let page_title_contains = page_title
                        .clone().map(|title| (title.clone(), title.contains(&cloned_pat)));

                    match page_title_contains {
                        Some(res) => {
                            // contains: print
                            if res.1 {
                                let joined = path.print_path(&res.0);
                                println!("Found: '{}'", joined);
                            }
                        },
                        None => ()
                    }

                    // reached limit
                    if copied_depth + 1 > depth_limit  {
                        println!("EXIT");
                        exit(0);
                    }

                    // add children
                    for child in child_hrefs {
                        // is_rel: diff logic
                        if is_relative(&child) {
                            let full_url = path.parsed_url.base.join(&child).unwrap();
                            let full_url = full_url.to_string();
                            let title = page_title.clone();

                            // shadow outer: need to clone for each task
                            let mut cloned_path = path.clone();


                            // visited in curr path already - skip
                            if !cloned_path.path_vis.contains(&full_url) {
                                cloned_path.path_array.push(title.unwrap_or("Empty Title".to_string()));
                                cloned_path.path_vis.insert(full_url);
                                cloned_path.parsed_url.relative = child;
                                cloned_path.depth += 1;

                                cloned_tx.send(cloned_path);
                            }

                            // check if full_url in vis set, skip if vis
                            // copy path array, append
                            // copy vis_set, add
                            // clone prev parsed_url and use
                            // depth+= 1


                        // TODO
                        } else {
                            // make a new parsed_url
                            // same logic for copy path array + copy vis_set-
                            let mut cloned_path = path.clone();
                        }
                        // cloned_path.depth+=1;
                    }


                    // match page_title {
                    //     Some(title) => {
                    //         if (title.contains(pattern)) {

                    //         }
                    //     },
                    //     None => {}
                    // }

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