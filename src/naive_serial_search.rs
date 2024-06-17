use crate::search_helpers::*;
use anyhow::Result;
use tokio::sync::mpsc;
use reqwest::Client;
use Message::*;

// Naive version of the search loop that doesn't take advantage of tokio spawn
// Time on jane.in: 373.881s
// Retained use of mpsc as queue to control for use of data structures
pub async fn naive_serial_search(url: &str, pattern: String, depth_limit: usize) -> Result<()> {
    let initial_path = Path::new(url)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    tx.send(PathRcv(initial_path))?;

    let client = Client::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            PathRcv(path) => {
                let most_recent_url = path.get_most_recent_url();
                let get_info = most_recent_url.get_info(client.clone()).await;

                match get_info {
                    Ok(info) => {
                        let page_title = info.page_title;
                        let child_hrefs = info.child_hrefs;
                        path.goal_test_on_title(
                            &page_title,
                            &pattern,
                            &most_recent_url.to_string(),
                        );

                        if path.depth < depth_limit {
                            for child in child_hrefs {
                                let new_url = most_recent_url.get_new_parsed_url(child)?;
                                if path.is_visited(&new_url) {
                                    continue;
                                }

                                let new_path =
                                    path.add_info(new_url, Some(most_recent_url.to_string()));
                                tx.send(PathRcv(new_path))?;
                            }
                        }

                        if path.depth + 1 == depth_limit {
                            rx.close();
                        }
                    }
                    Err(err) => {
                        eprintln!("ERROR: error requesting url - {}", err);
                    }
                }
            }
            Close => rx.close(),
        }
    }

    Ok(())
}

// Process may not stop - but doesn't use any sync mechanisms
// pub async fn search_without_stop(url:&str, pattern:String, depth_limit:usize)->Result<()> {
//     let initial_path = Path::new(url)?;
//     let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

//     // for initial MPSC send - need other tx to clone for remaining workers
//     let first_tx = tx.clone();

//     // send first path (task)
//     tokio::spawn(async move {
//         first_tx.send(PathRcv(initial_path));
//     });

//     while let Some(msg) = rx.recv().await {
//         match msg {
//             PathRcv(path) => {
//                 // println!("PATH_RECV: {}", path.to_string());

//                 // without clone, pattern is moved in each iter so can't use again - TODO: fix by using Rc
//                 let cloned_pattern = pattern.clone();
//                 // let sync = Arc::clone(&sync); // shadow
//                 let tx = tx.clone(); // shadow

//                 tokio::spawn(async move {

//                     let most_recent_url = path.get_most_recent_url(); // most recent url added to path
//                     // network request -> all child hrefs, page_title (Option since may not exist)
//                         // TODO: add sync++, sync-- here so that a slow request on one child doesn't get lost
//                     let get_info = most_recent_url.get_info().await;
//                     let curr_depth = path.depth;

//                     match get_info {
//                         Ok(info) => {
//                             let page_title = info.page_title;
//                             let child_hrefs = info.child_hrefs;

//                            // goal test, print path out if ok
//                            path.goal_test_on_title(&page_title, &cloned_pattern, &most_recent_url.to_string());

//                             // this check is done here instead of outside because of goal test
//                             if curr_depth < depth_limit {
//                                 // if child_depth == limit: sync++
//                                 for child in child_hrefs {
//                                     let get_new_parsed = most_recent_url.get_new_parsed_url(child).ok();
//                                     // true when err on parse -> skip this child
//                                     let is_vis = get_new_parsed
//                                     .clone()
//                                     .map(|url| path.is_visited(&url))
//                                     .unwrap_or(true);

//                                     if is_vis {
//                                         continue;
//                                     }

//                                     // make a new path and add to queue, increase sync for leaf (depth == limit)
//                                     match get_new_parsed {
//                                         Some(url) => {
//                                             // let new_title = page_title.clone();
//                                             let new_title = Some(most_recent_url.to_string());
//                                             // new_title: title of parent of this path
//                                             let new_path = path.add_info(url, new_title);

//                                             // add to queue, sync++ if leaf and send was successful
//                                             match tx.send(PathRcv(new_path)) {
//                                                 Ok(_) => {
//                                                     // if curr_depth + 1 == depth_limit {
//                                                     //     let mut sync_num = sync.lock().unwrap();
//                                                     //     *sync_num += 1;
//                                                     // }
//                                                 },
//                                                 Err(err) => {
//                                                     // println!("ERROR: error sending path into queue - {}", err.to_string())
//                                                 }
//                                             }
//                                         },
//                                         // ignore if parse error on absolute
//                                         None => ()
//                                     }

//                                 }
//                             }

//                             // done spawning
//                         },

//                         // handle error. e.g bad url
//                         Err(err) => {
//                             // println!("ERROR: error requesting url - {}", err.to_string());
//                         }
//                     }
//                 });
//             },

//             // close rcv - need to send msg because rx.close() not possible within individual tokio task since rx is single consumer
//             Close => {
//                 rx.close();
//             }
//         }
//     }

//     Ok(())
// }
