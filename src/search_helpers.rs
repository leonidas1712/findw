use crate::url_helpers::{parse_base_url, ParsedUrl};
use anyhow::Result;
use std::{collections::HashSet, fmt::Display};

/// Message from tx -> rx
pub enum Message {
    PathRcv(Path),
    Close,
}

// TODO: Change to use &str where possible, change path_vis to contain just string hashes and cmp on that (perf opt?)
/// Represents a node in the MPSC queue: a search path
#[derive(Clone)]
pub struct Path {
    pub depth: usize,
    /// page titles so far (EXCLUDING latest_url) for full path printing.
    // exclude because at Path creation we have not done the req yet.
    // TODO: modify to support grep on contents (need to store HTML content strings or objects)
    pub contents_array: Vec<String>,
    /// visited set so far (INCLUDING latest_url) to avoid cycles. for now, store full_url as String.
    // TODO: Can use relative with lifetimes, &ref
    pub path_vis: HashSet<ParsedUrl>,
    /// latest_url in path. extra variable for this because HashSet doesn't preserve insertion order
    // TODO: use lifetimes so this can be a reference instead
    pub latest_url: ParsedUrl,
}

// Methods take extra argument for latest so we can avoid adding to array/set first
impl Path {
    /// Create new path from given URL
    pub fn new(url: &str) -> Result<Path> {
        let latest_url = parse_base_url(url)?;
        let mut path_vis = HashSet::new();
        path_vis.insert(latest_url.clone()); // clone because value is moved

        Ok(Path {
            depth: 0,
            contents_array: vec![],
            path_vis,
            latest_url,
        })
    }

    /// Returns new Path with information added
    pub fn add_info(&self, new_parsed_url: ParsedUrl, page_title: Option<String>) -> Path {
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
    pub fn get_most_recent_url(&self) -> &ParsedUrl {
        &self.latest_url
    }

    /// Returns true if latest_url is already in visited set
    pub fn is_visited(&self, new_url: &ParsedUrl) -> bool {
        self.path_vis.contains(new_url)
    }

    /// Join current path titles with newest, because we only get newest upon get request
    pub fn print_path(&self, latest_title: &str) -> String {
        let joined: Vec<String> = self
            .contents_array
            .iter()
            .map(|url| url.to_string())
            .collect();
        let joined = joined.join(" => ");

        if joined.is_empty() {
            latest_title.to_string()
        } else {
            joined + " => " + latest_title
        }
    }

    /// Perform goal test on path given title Option. If passes, print concatenated path.
    pub fn goal_test_on_title(&self, title: &Option<String>, pattern: &str, string_to_print: &str) {
        match title {
            Some(title_string) => {
                // goal test passed
                if title_string.contains(pattern) {
                    let to_print = self.print_path(string_to_print);
                    // println!("Found: {}", to_print);
                    println!("{}", to_print);
                }
            }
            None => (),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined_arr: String = self.contents_array.iter().map(|s| s.to_string()).collect();

        let mut set_str: Vec<String> = self.path_vis.iter().map(|s| s.to_string()).collect();
        set_str.sort();

        let joined_arr = if joined_arr.is_empty() {
            String::from("[]")
        } else {
            joined_arr
        };

        let set_str = if set_str.is_empty() {
            String::from("{}")
        } else {
            format!("[{}]", set_str.join(", "))
        };

        write!(
            f,
            "(depth: {}, path: {}, vis:{}, latest_url:{})",
            self.depth, joined_arr, set_str, self.latest_url
        )
    }
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
