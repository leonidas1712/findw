use std::{sync::{Arc,Mutex}, collections::HashSet};
use anyhow::{Result};
use tokio::sync::mpsc;
use crate::{search_helpers::*, consts};
use Message::*;

// Improvements from Sep 15
// Program should stop when all tx go out of scope, but first tx has no chance to get dropped due to clone
    // Current fix: use Arc<Mutex> to track last level nodes then call rx.close()
pub async fn search2(url:&str, pattern:String, depth_limit:usize, print_title:bool)->Result<()> {
    let initial_path = Path::new(url)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();    
    
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
                    let get_info = most_recent_url.get_info().await;
                    let curr_depth = path.depth;    

                    match get_info {
                        Ok(info) => {
                            let page_title = info.page_title;
                            let child_hrefs = info.child_hrefs;

                            // if most_recent_url.to_string().contains("wrought") {
                            //     println!("self url, pg title: {}, {:?}", most_recent_url.to_string(),page_title);
                            // }

                            if print_title {
                                let title_print = page_title.clone().unwrap_or(String::from(consts::EMPTY_TITLE));
                                path.goal_test_on_title(&page_title, &cloned_pattern, &title_print);
                            } else {
                                path.goal_test_on_title(&page_title, &cloned_pattern, &most_recent_url.to_string());
                            }
        
                            // goal test, print path out if ok
                            
                            // this check is done here instead of outside because of goal test
                            
                            if curr_depth < depth_limit {
                                let mut sync_num = sync.lock().unwrap();
                                let mut vis_hrefs:HashSet<String> = HashSet::new();

                                for child in child_hrefs {
                                    let get_new_parsed = most_recent_url.get_new_parsed_url(child.clone()).ok();

                                    // true when err on parse -> skip this child
                                    let is_vis = get_new_parsed
                                    .clone()
                                    .map(|url| path.is_visited(&url))
                                    .unwrap_or(true);

                                    // this check is against the path so far
                                    if is_vis {
                                        continue;
                                    }

                                    // don't visit the same link on the page twice (unnecessary work)
                                    if vis_hrefs.contains(&child) {
                                        continue;
                                    }

                                    vis_hrefs.insert(child);
                                    
                                    // make a new path and add to queue, increase sync for leaf (depth == limit)
                                    match get_new_parsed {
                                        Some(url) => {
                                            let new_title = if print_title {
                                                page_title.clone()
                                            } else {
                                                Some(most_recent_url.to_string())
                                            };

                                            let new_path = path.add_info(url.clone(), new_title);

                                            // add to queue, sync++ if leaf and send was successful
                                            match tx.send(PathRcv(new_path)) {
                                                Ok(_) => {
                                                    // let mut sync_num = sync.lock().unwrap();
                                                    *sync_num += 1;
                                                    
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

                    // decrement and check nodes == 0 upon exit
                    let mut sync_num = sync.lock().unwrap();
                    *sync_num -= 1;

                    // no more last level threads left: send Close msg
                    if *sync_num == 0 {
                        tx.send(Message::Close);
                    }
                }); // end of tokio::spawn
            },

            // close rcv - need to send msg because rx.close() not possible within individual tokio task since rx is single consumer
            Close => {
                rx.close();
            }
        }
    }

    Ok(())
}