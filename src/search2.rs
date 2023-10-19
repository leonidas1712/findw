use std::{collections::HashSet, fmt::{Display}, sync::{Arc,Mutex}};
use anyhow::{Result};
use tokio::sync::mpsc;
use crate::url_helpers::{parse_base_url, ParsedUrl};

/// Message from tx -> rx
enum Message {
    PathRcv(Path),
    Close
}

fn print_set<T:Display>(set:&HashSet<T>)->String{
    let mut s = String::from("");

    for elem in set.iter() {
        s += &elem.to_string();
        s+= ",";
    }

    s
}

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
        // TODO: use lifetimes so this can be a reference instead
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

    /// Returns new Path with information added
    pub fn add_info(&self, new_parsed_url:ParsedUrl, page_title:Option<String>)->Path{
        let mut new_path = self.clone();
        let latest_url = new_parsed_url.clone(); // clone because not using lifetimes/ref for latest_url field yet

        new_path.depth += 1;
        new_path.path_vis.insert(new_parsed_url);

        if let Some(title) = page_title {
            new_path.contents_array.push(title);
        }

        new_path.latest_url = latest_url;
        new_path
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

    /// Perform goal test on path given title Option. If passes, print concatenated path.
    pub fn goal_test_on_title(&self, title:&Option<String>, pattern:&str) {
        
        match title {
            Some(title_string) => {
                // goal test passed
                if title_string.contains(pattern) {
                    let to_print = self.print_path(&title_string);
                    println!("Found: {}", to_print);
                    // println!("VIS_SET:{}", print_set(&self.path_vis));
                    // println!("");
                }
            },
            None => ()
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined_arr:String = self.contents_array.iter().map(|s| s.to_string()).collect();

        let mut set_str:Vec<String> = self.path_vis.iter().map(|s| s.to_string()).collect();
        set_str.sort();

        let joined_arr = if joined_arr.len() == 0 {
            String::from("[]")
        } else {
            joined_arr
        };

        let set_str = if set_str.len() == 0 {
            String::from("{}")
        } else {
            format!("[{}]", set_str.join(", "))
        };
    
        write!(f, "(depth: {}, path: {}, vis:{}, latest_url:{})", self.depth, joined_arr, set_str, self.latest_url.to_string())
    }
}

// Improvements from Sep 15
// Program should stop when all tx go out of scope, but first tx has no chance to get dropped due to clone
    // Current fix: use Arc<Mutex> to track last level nodes then call rx.close()
    // but this breaks when depth_limit != actual max depth of graph
use Message::*;
pub async fn search2(url:&str, pattern:String, depth_limit:usize)->Result<()> {
    let initial_path = Path::new(url)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    println!("Starting search with: {}\n", initial_path);

    // for initial MPSC send - need other tx to clone for remaining workers
    let first_tx = tx.clone();

    // send first path (task)
    tokio::spawn(async move {
        first_tx.send(PathRcv(initial_path));
    });

    // sync last level threads: when it reaches 0, rx.close()
    let sync:Arc<Mutex<usize>> = Arc::new(Mutex::new(1));

    while let Some(msg) = rx.recv().await {
        match msg {
            PathRcv(path) => {
                // println!("PATH_RECV: {}", path.to_string());

                // without clone, pattern is moved in each iter so can't use again - TODO: fix by using Rc
                let cloned_pattern = pattern.clone();
                let sync = Arc::clone(&sync); // shadow
                let tx = tx.clone(); // shadow
        
                tokio::spawn(async move {

                    let most_recent_url = path.get_most_recent_url(); // most recent url added to path
                    // network request -> all child hrefs, page_title (Option since may not exist)
                        // TODO: add sync++, sync-- here so that a slow request on one child doesn't get lost
                    let get_info = most_recent_url.get_info().await;
                    let curr_depth = path.depth;

        
                    match get_info {
                        Ok(info) => {
                            let page_title = info.page_title;
                            let child_hrefs = info.child_hrefs;
        
                            // goal test, print path out if ok
                            path.goal_test_on_title(&page_title, &cloned_pattern);
                            
                            // this check is done here instead of outside because of goal test
                            if curr_depth < depth_limit {
                                // if child_depth == limit: sync++
                                for child in child_hrefs {
                                    let get_new_parsed = most_recent_url.get_new_parsed_url(child).ok();
                                    // true when err on parse -> skip this child
                                    let is_vis = get_new_parsed
                                    .clone()
                                    .map(|url| path.is_visited(&url))
                                    .unwrap_or(true);

                                    if is_vis {
                                        continue;
                                    }
                                    
                                    // make a new path and add to queue, increase sync for leaf (depth == limit)
                                    match get_new_parsed {
                                        Some(url) => {
                                            let new_title = page_title.clone();
                                            let new_path = path.add_info(url, new_title);

                                            // add to queue, sync++ if leaf and send was successful
                                            match tx.send(PathRcv(new_path)) {
                                                Ok(_) => {
                                                    if curr_depth + 1 == depth_limit {
                                                        let mut sync_num = sync.lock().unwrap();
                                                        *sync_num += 1;
                                                    }
                                                },
                                                Err(err) => {
                                                    // println!("ERROR: error sending path into queue - {}", err.to_string())
                                                }
                                            }
                                        },
                                        // ignore if parse error on absolute
                                        None => ()
                                    }

                                }
                            }
                            
        
                            // done spawning
                        },
                        
                        // handle error. e.g bad url
                        Err(err) => {
                            // println!("ERROR: error requesting url - {}", err.to_string());
                        }
                    }

                    // reached depth_limit: sync--, then check if 0 => rx.close
                    // why outside match: if match runs error branch below should still run
                    if curr_depth == 0 || curr_depth == depth_limit {
                        let mut sync_num = sync.lock().unwrap();
                        *sync_num -= 1;

                        // no more last level threads left: send Close msg
                        if *sync_num == 0 {
                            tx.send(Message::Close);
                        }
                    }
        
                });
            },

            // close rcv - need to send msg because rx.close() not possible within individual tokio task since rx is single consumer
            Close => {
                println!("Closed");
                rx.close();
            }
        }
    }

    Ok(())
}

/// Process may not stop
pub async fn search_without_stop(url:&str, pattern:String, depth_limit:usize)->Result<()> {
    let initial_path = Path::new(url)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    println!("Starting search with: {}\n", initial_path);

    // for initial MPSC send - need other tx to clone for remaining workers
    let first_tx = tx.clone();

    // send first path (task)
    tokio::spawn(async move {
        first_tx.send(PathRcv(initial_path));
    });

    // sync last level threads: when it reaches 0, rx.close()
    // let sync:Arc<Mutex<usize>> = Arc::new(Mutex::new(1));

    while let Some(msg) = rx.recv().await {
        match msg {
            PathRcv(path) => {
                // println!("PATH_RECV: {}", path.to_string());

                // without clone, pattern is moved in each iter so can't use again - TODO: fix by using Rc
                let cloned_pattern = pattern.clone();
                // let sync = Arc::clone(&sync); // shadow
                let tx = tx.clone(); // shadow
        
                tokio::spawn(async move {

                    let most_recent_url = path.get_most_recent_url(); // most recent url added to path
                    // network request -> all child hrefs, page_title (Option since may not exist)
                        // TODO: add sync++, sync-- here so that a slow request on one child doesn't get lost
                    let get_info = most_recent_url.get_info().await;
                    let curr_depth = path.depth;

        
                    match get_info {
                        Ok(info) => {
                            let page_title = info.page_title;
                            let child_hrefs = info.child_hrefs;
        
                            // goal test, print path out if ok
                            path.goal_test_on_title(&page_title, &cloned_pattern);
                            
                            // this check is done here instead of outside because of goal test
                            if curr_depth < depth_limit {
                                // if child_depth == limit: sync++
                                for child in child_hrefs {
                                    let get_new_parsed = most_recent_url.get_new_parsed_url(child).ok();
                                    // true when err on parse -> skip this child
                                    let is_vis = get_new_parsed
                                    .clone()
                                    .map(|url| path.is_visited(&url))
                                    .unwrap_or(true);

                                    if is_vis {
                                        continue;
                                    }
                                    
                                    // make a new path and add to queue, increase sync for leaf (depth == limit)
                                    match get_new_parsed {
                                        Some(url) => {
                                            let new_title = page_title.clone();
                                            let new_path = path.add_info(url, new_title);

                                            // add to queue, sync++ if leaf and send was successful
                                            match tx.send(PathRcv(new_path)) {
                                                Ok(_) => {
                                                    // if curr_depth + 1 == depth_limit {
                                                    //     let mut sync_num = sync.lock().unwrap();
                                                    //     *sync_num += 1;
                                                    // }
                                                },
                                                Err(err) => {
                                                    // println!("ERROR: error sending path into queue - {}", err.to_string())
                                                }
                                            }
                                        },
                                        // ignore if parse error on absolute
                                        None => ()
                                    }

                                }
                            }
                            
        
                            // done spawning
                        },
                        
                        // handle error. e.g bad url
                        Err(err) => {
                            // println!("ERROR: error requesting url - {}", err.to_string());
                        }
                    }

                    // reached depth_limit: sync--, then check if 0 => rx.close
                    // why outside match: if match runs error branch below should still run
                    // if curr_depth == 0 || curr_depth == depth_limit {
                    //     let mut sync_num = sync.lock().unwrap();
                    //     *sync_num -= 1;

                    //     // no more last level threads left: send Close msg
                    //     if *sync_num == 0 {
                    //         tx.send(Message::Close);
                    //     }
                    // }
        
                });
            },

            // close rcv - need to send msg because rx.close() not possible within individual tokio task since rx is single consumer
            Close => {
                println!("Closed");
                rx.close();
            }
        }
    }

    Ok(())
}


#[cfg(test)]
pub mod tests {
    use crate::url_helpers::parse_base_url;

    #[test]
    pub fn test_path_add_info() {
        let url = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
        let path = super::Path::new(&url).unwrap();

        let jane = "https://www.janestreet.com/";
        let new_url = parse_base_url(jane).unwrap();
        let add = new_url.clone();

        let path2 = path.add_info(new_url, Some(String::from("Home :: Jane Street")));
        assert_eq!(path2.to_string(), "(depth: 1, path: Home :: Jane Street, vis:[https://blog.janestreet.com/what-the-interns-have-wrought-2023/, https://www.janestreet.com/], latest_url:https://www.janestreet.com/)");
        
        path2.add_info(add, Some(String::from("Home :: Jane Street")));
        dbg!(path2.to_string());
    }
}