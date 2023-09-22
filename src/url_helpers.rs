use std::vec;

use url::{Url};
use anyhow::{anyhow, Result};

/// Get base e.g http://localhost:8000/index.html => http://localhost:8000 
/// and relative url: (base, relative)
/// Provides helper method for full url and parsing
pub struct ParsedUrl {
    // https://localhost:8000/ or https://blog.janestreet.com/
    pub base:Url, // TODO: change to use pointer (some collection in main passed down) to avoid .clone()
    /// about.html or what-the-interns-have-wrought-2023
    pub relative:String  
}

impl Clone for ParsedUrl {
    fn clone(&self) -> Self {
        ParsedUrl {
            base: self.base.clone(),
            relative: self.relative.clone()
        }
    }
}

/// Return result from get_info
// TODO: change to use &str where possible
pub struct InfoResult {
    pub child_hrefs: Vec<String>,
    pub page_title:Option<String>
}

impl ParsedUrl {
    pub fn get_full_url(&self)->String {
        self.base.join(&self.relative).unwrap().to_string()
    }

    // request full URL -> get child hrefs + document title
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
    
        // get hrefs raw
        let links = document.select(&link_selector);
        let links:Vec<_> = links.collect();

        let links:Vec<String> = links.into_iter().filter_map(|elem| {
            let val = elem.value();
            let href = val.attr("href").map(|val| val.to_string());
            return href;
        }).collect();

        println!("LINKS: {:?}", links);

        // into_iter because a selector can technically match many elems, but title tag we only look at first
        let title_select = document.select(&title_selector).into_iter().next(); 

        // if no title found, placeholder value
        let title = match title_select {
            Some(elem) => elem.inner_html(),
            None => { 
                // println!("EMPTY");
                String::from("(Empty title)") 
            }
        };

        // title found but empty - None option
        let opt_title = if title.len() == 0 {
            None
        } else {
            Some(title)
        };

        // ch
        Ok(InfoResult { child_hrefs: links, page_title: opt_title })

    }
}



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

    let scheme = parsed.scheme(); // http or https
    let domain = domain.unwrap(); // localhost or blog.janestreet.com
    let port = parsed.port(); // 8000
    let relative = parsed.path();

    // make into the right string
    let base_url = match port {
        Some(prt) => {
            format!("{scheme}://{domain}:{prt}")
        },

        None => {
            format!("{scheme}://{domain}")
        }
    };
    
    // parse back into Url
    let base_url = Url::parse(&base_url)?;
    Ok(ParsedUrl { base: base_url, relative: relative.to_string()})
}

pub fn debug_url(url:&str) {
    let parsed = Url::parse(url).unwrap();
    
    println!("Domain: {:?}", parsed.domain());
    println!("Scheme: {:?}", parsed.scheme());
    println!("Port:{:?}", parsed.port());
    println!("Path:{:?}", parsed.path());
    println!("");
}

#[cfg(test)]
pub mod tests {
    use super::parse_base_url;

    #[test]
    pub fn test_get_base_url() {
        let local = "http://localhost:8000/index.html";
        let res = parse_base_url(local);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!("http://localhost:8000/", res.base.to_string());
        assert_eq!("http://localhost:8000/index.html", res.get_full_url());


        let norm = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
        let res = parse_base_url(norm);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!("https://blog.janestreet.com/", res.base.to_string());
        assert_eq!("https://blog.janestreet.com/what-the-interns-have-wrought-2023/", res.get_full_url());

        // rel. without or with / is fine
        // println!("JOIN: {}", res.base.join("/about.html").unwrap().to_string());
    }

    #[test]
    pub fn test_get_base_url_err() {
        let url = "badurl";
        let res = parse_base_url(url);
        assert!(res.is_err());
    }
}