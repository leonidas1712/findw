use std::{collections::HashSet, fmt::Display};
use anyhow::{anyhow, Result};
use regex::Regex;
use tokio::sync::mpsc;


pub struct LinkNodeData {
    pub title:String,
    pub child_urls:Vec<String>
}

// https:// - return as is
// /wiki/ - expand
pub fn expand_url(url:&str)->String {
    let wiki = Regex::new(r"\/wiki\/(.*)").unwrap();
    let mut captures_try = wiki.captures(url);
    
    // matches /wiki/: use to expand URL, captures[1] is the name
    if let Some(captures) = captures_try {
        let captures:Vec<_> = captures.iter().map(|e| e.unwrap().as_str()).collect();
        if captures.len() >= 2 {
            let url = "https://en.wikipedia.org/wiki/";
            let name = captures.get(1).unwrap();
            return url.to_owned()+name.to_owned();
        }
    
        // for c in captures {
        //     println!("Capt:{}", c);
        // }
    }

    String::from(url)
}

impl LinkNodeData {
    // return a result from this
    fn get_data_from_html(html:&str)->Result<LinkNodeData>{
        // class specific to wiki (testing)
        let title_selector = scraper::Selector::parse("title").unwrap();

        // filtering some useless wiki classes
        let link_selector = scraper::Selector::parse("a:not(.interlanguage-link-target, .mw-jump-link)").unwrap();

        let document = scraper::Html::parse_document(&html);

        // document.title
        let title = document.select(&title_selector).into_iter().next().unwrap().inner_html();
        
        // document.querySelectorAll(a:not[...])
        let links = document.select(&link_selector);
        let links:Vec<_> = links.collect();
        println!("Links length:{}", links.len());

        // take the link, map to href -> expand href if needed
        let urls:Vec<String> = links.into_iter().filter_map(|elem| {
            let val = elem.value();
            let href = val.attr("href");

            // regex: https:// or /wiki/ 
                // if /wiki/ - expand and return Some(expanded)
            if let Some(url) = href {
                return Some(expand_url(url)); // todo expand 
            } else {
                // when it has no href attr
                return None;
            }

        }).collect();

        if title.len() == 0 || urls.len() == 0 {
            return Err(anyhow!("Empty link node data (title or urls)"))
        }

        Ok(LinkNodeData {
            title,
            child_urls:urls
        })
    }
}

/// Part of a given search Path
pub struct LinkNode {
    pub url: String,
    pub data: LinkNodeData
}   



impl LinkNode {
    // Result<LinkNode>
    pub async fn linknode_from_url(url:&str)->Result<LinkNode>{
        let html = reqwest::get(url)
        .await?
        .text()
        .await?;

        let data = LinkNodeData::get_data_from_html(&html)?;

        Ok(LinkNode {
            url:url.to_string(),
            data
        })
    }
}

impl Display for LinkNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {} child links)", self.url, self.data.title, self.data.child_urls.len())
    }
}

/// Search path along some given URLs - path list, visited set
pub struct Path {
    pub nodes: Vec<LinkNode>,
    /// use hrefs to track visited
    pub visited_hrefs: HashSet<String>
}

impl Path {
    pub fn new(node:LinkNode)->Path {
        let mut nodes:Vec<LinkNode> = vec![];
        let mut visited_hrefs:HashSet<String> = HashSet::new();

        visited_hrefs.insert(node.url.clone());
        nodes.push(node);

        Path {
            nodes,
            visited_hrefs
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nodes_str:Vec<String> = self.nodes.iter().map(|n| n.to_string()).collect();
        let nodes_str = nodes_str.join("=>");

        write!(f, "{}", nodes_str)
    }
}

/// Main search function. Stop when path length reaches limit (anything >= this ignore)
pub async fn search(url:&str, pattern:&str, limit:usize)->Result<String> {
    let init_node = LinkNode::linknode_from_url(url).await;
    if init_node.is_err() {
        return Err(anyhow!("Initial node with url {url} is invalid."));
    }

    let init_node = init_node.unwrap();
    let init_path = Path::new(init_node);

    let (tx, mut rx) = mpsc::unbounded_channel::<Path>();

    // send first path (task)
    tokio::spawn(async move {
        tx.send(init_path);
    });

    // main receiver (single consumer) in main thread
        // Ext: Use MPMC here? broadcast
    while let Some(path) = rx.recv().await {
        println!("Path recv: {}", path);
    }

    // println!("{},{}", init_path.nodes.len(), init_path.visited_hrefs.len());

    Ok(String::from("Search done"))
}














// ----------------------------
// Doc title:"Hello, world!"
// Got to links
// x content: Link to google 
// x name: a
// a: href, www.google.com
// x content: Link to google num 2 
// x name: a
// a: href, www.google.com

// /// Scrape ahrefs
pub fn get_links() {
    use scraper::{Html,Selector};

    let html = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>

        <h1 class="foo">Hello, <i>world!</i></h1>
        <a href="www.google.com"> Link to google </a>
        <a href="www.google.com"> Link to google num 2 </a>

    "#;

    let link_selector = Selector::parse("a").unwrap();
    let title_sel = Selector::parse("title").unwrap();

    let document = Html::parse_document(html);
    println!("Doc title:{:?}", document.select(&title_sel).into_iter().next().unwrap().inner_html());
    
    let links = document.select(&link_selector);
    println!("Got to links");

    let v:Vec<_> = links.into_iter().collect();
    // println!("Len: {}", v.len());
    v.iter().for_each(|x| {
        let x_val = x.value();

        println!("x content:{}", x.inner_html());
        println!("x name: {}", x_val.name());

        let attrs = x.value().attrs();
        
        // prints href, www.google.com
        attrs.for_each(|a| println!("a: {}, {}", a.0, a.1));

    });
}