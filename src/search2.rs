use std::{collections::HashSet, fmt::{Display}};
use anyhow::{Result};
use tokio::sync::mpsc;
use crate::url_helpers::{parse_base_url, ParsedUrl, is_relative};

// TODO: Change to use &str where possible, change path_vis to contain just string hashes and cmp on that (perf opt?)
/// Represents a node in the MPSC queue: a search path
#[derive(Clone)]
struct Path {
    pub depth:usize,
    /// page titles so far (EXCLUDING latest_url) for full path printing. 
        // exclude because at Path creation we have not done the req yet.
        // TODO: modify to support grep on contents (need to store HTML content strings or objects)
    pub contents_array: Vec<String>, 
    /// visited set so far (INCLUDING latest_url) to avoid cycles. for now, store full_url as String. 
        // TODO: Can use relative with lifetimes, &ref
    pub path_vis: HashSet<ParsedUrl>, 
    /// latest_url in path. extra variable for this because HashSet doesn't preserve insertion order
    pub latest_url:ParsedUrl 
}

// Methods take extra argument for latest so we can avoid adding to array/set first
impl Path {
    /// Create new path from given URL
    pub fn new(url:&str)->Result<Path> {
        let latest_url = parse_base_url(url)?;
        let mut path_vis = HashSet::new();
        path_vis.insert(latest_url.clone()); // clone because value is moved

        Ok(Path {
            depth:0,
            contents_array:vec![],
            path_vis,
            latest_url
        })
    }

    /// Returns latest_url in this path by insertion order
    pub fn get_most_recent_url(&self)->&ParsedUrl {
        &self.latest_url
    }

    /// Returns true if latest_url is already in visited set
    pub fn is_visited(&self, new_url:&ParsedUrl)->bool {
        self.path_vis.contains(new_url)
    }

    /// Join current path titles with newest, because we only get newest upon get request
    pub fn print_path(&self, latest_title:&str) -> String {
        let joined:Vec<String> = self.contents_array.iter().map(|url| url.to_string()).collect();
        let joined = joined.join(" => ");

        if joined.is_empty() {
            latest_title.to_string()
        } else {
            joined + " => " + latest_title
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined_arr:String = self.contents_array.iter().map(|s| s.to_string()).collect();
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
    
        write!(f, "(d: {}, path: {}, vis:{})", self.depth, joined_arr, set_str)
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


    Ok(())
}


// Improvements from Sep 15
// Program stops when all tx go out of scope
    // Eventually children are no longer added so no more txs to clone - all dropped
// pub async fn search2(url:&str, pattern:String, depth_limit:usize)->Result<()> {
//     let initial_path = Path::new(url)?;
//     let (tx, mut rx) = mpsc::unbounded_channel::<Path>();
//     println!("Starting search with: {}\n", initial_path);

//     // for initial MPSC send - need other tx to clone for remaining workers
//     let first_tx = tx.clone();

//     // send first path (task)
//     tokio::spawn(async move {
//         first_tx.send(initial_path);
//     });

//     while let Some(path) = rx.recv().await {
//         // parent depth, stop if +1 > limit
//         let copied_depth = path.depth;

//         // cloned path for new task - Strat 1
//         // let mut cloned_path = path.clone();
//         let cloned_tx = tx.clone();
//         let cloned_pat = pattern.clone();

//         tokio::spawn(async move {
//             // BLOCKING WITHIN TASK: add children nodes to mpsc - spawn new tasks
//             let get_info = path.parsed_url.get_info().await;
            
//             match get_info {
//                 Ok(info) => {
//                     // get title, hrefs
//                     let page_title = info.page_title;
//                     let child_hrefs = info.child_hrefs;


//                     // GOAL TEST AND PRINT: right now just title.contains

//                     // map to (title, bool) where bool=T means title meets goal test
//                     let page_title_contains = page_title
//                         .clone().map(|title| (title.clone(), title.contains(&cloned_pat)));

//                     match page_title_contains {
//                         Some(res) => {
//                             // contains: print
//                             if res.1 {
//                                 let joined = path.print_path(&res.0);
//                                 println!("Found: '{}'", joined);
//                             }
//                         },
//                         None => ()
//                     }

//                     // reached limit - just stop
//                     if copied_depth + 1 > depth_limit  {
//                         return;
//                     }

//                     // add children
//                     for child in child_hrefs {
//                         // is_rel: diff logic
//                         if is_relative(&child) {
//                             let full_url = path.parsed_url.base.join(&child).unwrap();
//                             let full_url = full_url.to_string();
//                             let title = page_title.clone();

//                             // shadow outer: need to clone for each task
//                             let mut cloned_path = path.clone();


//                             // visited in curr path already - skip
//                             if !cloned_path.path_vis.contains(&full_url) {
//                                 cloned_path.path_array.push(title.unwrap_or("Empty Title".to_string()));
//                                 cloned_path.path_vis.insert(full_url);
//                                 cloned_path.parsed_url.relative = child;
//                                 cloned_path.depth += 1;

//                                 cloned_tx.send(cloned_path);
//                             }
//                         } else {
//                             // make a new parsed_url
//                             // same logic for copy path array + copy vis_set-
//                             let mut cloned_path = path.clone();
//                             let full_url = child;
//                             let title = page_title.clone();


//                             if !cloned_path.path_vis.contains(&full_url) {
//                                 cloned_path.path_array.push(title.unwrap_or("Empty Title".to_string()));
//                                 cloned_path.path_vis.insert(full_url.clone());

//                                 // new parsedurl
//                                 let parsed_url = parse_base_url(&full_url);

//                                 // if err ignore
//                                 match parsed_url {
//                                     Ok(url) => {
//                                         cloned_path.parsed_url = url;
//                                         cloned_path.depth += 1;
//                                         cloned_tx.send(cloned_path);

//                                     },
//                                     _ => ()
//                                 }
//                             }
//                         }
//                         // cloned_path.depth+=1;
//                     }
//                 },
//                 // handle task failure: print error
//                 Err(err) => {
//                     println!("ERROR: {}", err);
//                 }
//             }

//             // just pattern match (goal test) here, then only add children to queue
//        });
//     }

//     Ok(())
// }

