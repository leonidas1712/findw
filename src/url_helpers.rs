use std::{fmt::Display, hash::{Hash, Hasher}};
use url::{Url};
use anyhow::{anyhow, Result};

use crate::consts;

/// Return result from ParsedUrl.get_info
// TODO: change to use &str where possible
pub struct InfoResult {
    /// all href tags from children <a href> - can either be absolute or relative URL
    pub child_hrefs: Vec<String>,
    /// title of page from <title> tag - can be None if page has no title tag
    pub page_title:Option<String>
}

impl Display for InfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty = "EMPTY TITLE".to_string();
        write!(f, "(title: '{}', children: {:?})", self.page_title.as_ref().unwrap_or(&empty), self.child_hrefs)
    }
}

/// Get base e.g http://localhost:8000/index.html => http://localhost:8000 
/// and relative url: (base, relative).
/// Provides helper method for full url and parsing
#[derive(Debug, Eq)]
pub struct ParsedUrl {
    // https://localhost:8000/ or https://blog.janestreet.com/; Url comes from url crate
    pub base:Url, // TODO: change to use pointer (some collection in main passed down) to avoid .clone()
}

impl Clone for ParsedUrl {
    fn clone(&self) -> Self {
        ParsedUrl {
            base: self.base.clone(),
        }
    }
}

impl PartialEq for ParsedUrl {
    fn eq(&self, other: &Self) -> bool {
        let other_str = other.get_full_url();
        self.get_full_url().eq(&other_str)
    }
}

impl Hash for ParsedUrl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let string = self.get_full_url();
        string.hash(state);
    }
}

impl Display for ParsedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_full_url())
    }
}

impl ParsedUrl {
    /// Return URL without fragment (if fragment is there) - to prevent duplicate URLs due to fragments
    pub fn get_full_url(&self)->String {
        let url = self.base.to_string();
        if self.base.fragment().is_some() {
            url.split("#").next().unwrap().to_string()
        }
        else {
            url
        }
    }

    /// request full URL -> get child hrefs + document title - to request within task
    // TODO: add individual tests for this
    pub async fn get_info(&self)->Result<InfoResult>{
        // TODO: Can't do const with runtime type - use thread_local?
        let title_selector = scraper::Selector::parse("title").unwrap();
        // classes are to filter on wikipedia useless links
        let link_selector = scraper::Selector::parse("a:not(.interlanguage-link-target, .mw-jump-link)").unwrap();

        let url = self.get_full_url();
        let html = reqwest::get(url)
            .await?
            .text()
            .await?;

        let document = scraper::Html::parse_document(&html);
    
        // get hrefs raw as strings
        let links = document.select(&link_selector);
        let links:Vec<_> = links.collect();

        let links:Vec<String> = links.into_iter().filter_map(|elem| {
            let val = elem.value();
            let href = val.attr("href").map(|val| val.to_string());
            return href;
        }).collect();

        // into_iter because a selector can technically match many elems, but title tag we only look at first
        let title_select = document.select(&title_selector).into_iter().next(); 

        // if no title found, placeholder value
        let title = match title_select {
            Some(elem) => elem.inner_html(),
            None => { 
                String::from(consts::EMPTY_TITLE) 
            }
        };

        // title found but empty - None option
        let opt_title = if title.len() == 0 {
            None
        } else {
            Some(title)
        };

        Ok(InfoResult { child_hrefs: links, page_title: opt_title })
    }
    
    /// Get new parsed_url based on whether child_href is relative or not - url.join handles this
    pub fn get_new_parsed_url(&self, child_href:String)->Result<ParsedUrl> {
        let new_url = self.base.join(&child_href);
        let mut new_parsed = self.clone();

        match new_url {
            Ok(res) => {
                new_parsed.base = res;
                Ok(new_parsed)
            },
            Err(err) => {
                Err(anyhow!(err))
            }
        } 
    }
}


/// Input: Full url string with scheme, domain, port etc. Output: ParsedUrl with base, relative separated
pub fn parse_base_url(url:&str)->Result<ParsedUrl> {
    let parsed = Url::parse(url);
    if parsed.is_err() {
        return Err(anyhow!("Invalid url - {}", url))
    }
    
    let parsed = parsed.unwrap();
    let domain = parsed.domain();

    if domain.is_none() {
        return Err(anyhow!("URL '{}' has no domain.", url));
    }

    Ok(ParsedUrl { base: parsed })
}

/// Return true if URL is relative, else false
// TODO: make this more robust
pub fn is_relative(url:&str)->bool {
    !(url.starts_with("http://") || url.starts_with("https://"))
}

pub fn debug_url(url:&str) {
    let parsed = Url::parse(url).unwrap();
    
    println!("Domain: {:?}", parsed.domain());
    println!("Scheme: {:?}", parsed.scheme());
    println!("Port:{:?}", parsed.port());
    println!("Path:{:?}", parsed.path());
    println!("");
}

use super::search_helpers::Path;
// function to print all child hrefs for a URL
pub async fn debug_url_hrefs(url:&str)->anyhow::Result<()>{
    let path = Path::new(url)?;
    let url = path.get_most_recent_url();
    let info = url.get_info().await;

    match info {
        Ok(res) => {
            let hrefs = res.child_hrefs;
            // println!("no. of children: {}", hrefs.len());
            hrefs.iter().for_each(|s| println!("{}", s));

            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn debug_url_hrefs_joined(url:&str)->anyhow::Result<()>{
    let path = Path::new(url)?;
    let url_obj = path.get_most_recent_url();
    let info = url_obj.get_info().await;

    match info {
        Ok(res) => {
            let hrefs = res.child_hrefs;
            // println!("no. of children: {}", hrefs.len());

            let url = url::Url::parse(url).unwrap();
            // println!("URL: {}", url.to_string());
            hrefs.iter().for_each(|s| {
                let join = url.join(s);
                match join {
                    Ok(joined_url) => {
                        println!("{}", joined_url.to_string());
                    }, 
                    Err(_) => {
                        println!("ERR: couldn't join child:{}", s);
                    }
                }
            });

            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{url_helpers::is_relative};
    use super::parse_base_url;

    #[test]
    pub fn test_get_base_url() {
        let local = "http://localhost:8000/index.html";
        let res = parse_base_url(local);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!("http://localhost:8000/index.html", res.get_full_url());


        let norm = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
        let res = parse_base_url(norm);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!("https://blog.janestreet.com/what-the-interns-have-wrought-2023/", res.get_full_url());

        // nested relative
        let res2 = parse_base_url("https://blog.janestreet.com/author/yminsky/").unwrap();
        assert_eq!(&res2.get_full_url(), "https://blog.janestreet.com/author/yminsky/");

        // rel. without or with / is fine
        // println!("JOIN: {}", res.base.join("/about.html").unwrap().to_string());
    }

    #[test]
    pub fn test_get_base_url_err() {
        let url = "badurl";
        let res = parse_base_url(url);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_is_relative() {
        assert_eq!(is_relative("http://localhost:8000/info.html"), false);
        assert_eq!(is_relative("https://blog.janestreet.com/what-the-interns-have-wrought-2023/"), false);
        assert_eq!(is_relative("info.html"), true);
        assert_eq!(is_relative("/info.html"), true);
    }

    #[test]
    pub fn test_get_new_parsed_url() {
        let loc = "http://localhost:8000/info.html";
        let url = parse_base_url(loc).unwrap();
        let url2 = url.get_new_parsed_url(String::from("about.html")).unwrap();
        assert_eq!(url2.to_string(),"http://localhost:8000/about.html");

        // should ignore if absolute i.e just call parse_base
        let new_href = String::from("https://blog.janestreet.com/what-the-interns-have-wrought-2023/");
        let url2 = url.get_new_parsed_url(new_href).unwrap();
        assert_eq!(url2.to_string(), "https://blog.janestreet.com/what-the-interns-have-wrought-2023/");

    }

    #[test]
    pub fn test_hash_parsed_url() {
        use std::collections::HashSet;
        let url_str = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
        let url = parse_base_url(&url_str).unwrap();

        let url_copy = parse_base_url(&url_str).unwrap();

        let url2 = "https://www.janestreet.com/";
        let url2 = parse_base_url(url2).unwrap();

        let mut h:HashSet<super::ParsedUrl> = HashSet::new();

        h.insert(url);
        h.insert(url_copy);
        h.insert(url2.clone());
        h.insert(url2);


        assert_eq!(h.len(), 2);
    }   
}