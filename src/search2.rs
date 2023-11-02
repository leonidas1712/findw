use std::{sync::{Arc,Mutex}};
use anyhow::{Result};
use tokio::sync::mpsc;
use crate::search_helpers::*;
use Message::*;

// Improvements from Sep 15
// Program should stop when all tx go out of scope, but first tx has no chance to get dropped due to clone
    // Current fix: use Arc<Mutex> to track last level nodes then call rx.close()
    // but this breaks when depth_limit != actual max depth of graph
pub async fn search2(url:&str, pattern:String, depth_limit:usize)->Result<()> {
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
                        // TODO: add sync++, sync-- here so that a slow request on one child doesn't get lost
                    let get_info = most_recent_url.get_info().await;
                    let curr_depth = path.depth;

        
                    match get_info {
                        Ok(info) => {
                            let page_title = info.page_title;
                            let child_hrefs = info.child_hrefs;
        
                            // goal test, print path out if ok
                            path.goal_test_on_title(&page_title, &cloned_pattern, &most_recent_url.to_string());
                            
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
                                            // new_title: title of parent of this path
                                            // let new_title = page_title.clone();
                                            let new_title = Some(most_recent_url.to_string());
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