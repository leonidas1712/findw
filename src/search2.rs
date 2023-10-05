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
    pub fn goal_test_on_title(&self, title:Option<String>, pattern:&str) {
        match title {
            Some(title_string) => {
                // goal test passed
                if title_string.contains(pattern) {
                    let to_print = self.print_path(&title_string);
                    println!("Found: {}", to_print);
                }
            },
            None => ()
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined_arr:String = self.contents_array.iter().map(|s| s.to_string()).collect();
        let set_str:Vec<String> = self.path_vis.iter().map(|s| s.to_string()).collect();

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
    
        write!(f, "(d: {}, path: {}, vis:{}, latest_url:{})", self.depth, joined_arr, set_str, self.latest_url.to_string())
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
        println!("PATH_RECV: {}", path.to_string());

        // without clone, pattern is moved in each iter so can't use again
            // TODO: fix by using Rc
        let cloned_pattern = pattern.clone();

        tokio::spawn(async move {
            let most_recent_url = path.get_most_recent_url(); // most recent url added to path
            // network request -> all child hrefs, page_title (Option since may not exist)
            let get_info = most_recent_url.get_info().await;

            match get_info {
                Ok(info) => {
                    let page_title = info.page_title;
                    let child_hrefs = info.child_hrefs;

                    // goal test, print path out if ok
                    path.goal_test_on_title(page_title, &cloned_pattern);

                    // return out if children are above depth_limit
                    if path.depth + 1 > depth_limit {
                        return;
                    }


                },
                
                // handle error. e.g bad url
                Err(err) => {
                    println!("ERROR: {}", err.to_string());
                }
            }

        });

        // rx.close();
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
        let path2 = path.add_info(new_url, Some(String::from("Home :: Jane Street")));
        assert_eq!(path2.to_string(), "(d: 1, path: Home :: Jane Street, vis:[https://blog.janestreet.com/what-the-interns-have-wrought-2023/, https://www.janestreet.com/], latest_url:https://www.janestreet.com/)");
    }
}