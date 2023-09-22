use std::{collections::HashSet, fmt::{Display, format, Debug}};
use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::node;
use tokio::sync::mpsc;


#[derive(Clone)]
pub struct LinkNodeData {
    pub title:Option<String>,
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
        
        let links = document.select(&link_selector);
        let links:Vec<_> = links.collect();

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

        // document.title TODO: fix this unwrap
        // into_iter because a selector can technically match many elems, but title tag we only look at first
        let title_select = document.select(&title_selector).into_iter().next(); 
        
        let title = match title_select {
            Some(elem) => elem.inner_html(),
            None => { 
                // println!("EMPTY");
                String::from("(Empty title)") 
            }
        };

        // if title.len() == 0 || urls.len() == 0 {
        //     return Err(anyhow!("Empty link node data (title or urls)"))
        // }

        let opt_title = if title.len() == 0 {
            None
        } else {
            Some(title)
        };

        Ok(LinkNodeData {
            title:opt_title,
            child_urls:urls
        })
    }
}

/// Part of a given search Path
#[derive(Clone)]
pub struct LinkNode {
    pub url: String, // full url
    pub data: LinkNodeData // 
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

impl Debug for LinkNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title_out:&str = match &self.data.title {
            Some(title) => title.as_ref(),
            None => "Empty Title"
    
        };

        write!(f, "({}, {}, {} child links)", self.url, title_out, self.data.child_urls.len())
    }
}

impl Display for LinkNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self.data.title.as_ref();
        let title = match title {
            Some(s) => s,
            None => "Empty Title"
        };

        write!(f, "{}", title)
    }
}

/// Search path along some given URLs - path list, visited set
#[derive(Clone)]
pub struct Path {
    pub nodes: Vec<LinkNode>,
    /// use hrefs to track visited
    pub visited_hrefs: HashSet<String> // TODO: check BTreeMap, or other alternatives
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

    pub fn add(&mut self, node:LinkNode) {
        self.visited_hrefs.insert(node.url.clone());
        self.nodes.push(node);
    }
}

// Join title strings
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str:Vec<String> = vec![];

        for node in self.nodes.iter() {
            let title = node.to_string();
            out_str.push(title);
        }

        let output = out_str.join(" => ");
        

        write!(f, "{}", output)
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

    let first_tx = tx.clone();
    // send first path (task)
    tokio::spawn(async move {
        first_tx.send(init_path);
    });

    // main receiver (single consumer) in main thread
        // TODO: Use MPMC here? broadcast
    while let Some(path) = rx.recv().await {
        // println!("Path recv: {}", path);
        let most_recent = path.nodes.last().unwrap();

        // goal test - TODO: replace with regex l8r
        // move to per thread
        if most_recent.data.title.clone().map(|s| s.contains(pattern)).unwrap_or(false) {
            let res = format!("{}", path.to_string());
            println!("{res}"); // TODO: some kind of async output? instead of forcing buffer to flush immediately
        }

        let recent = most_recent.clone();

        // for each child_url, make a new task to process
        for child_url in recent.data.child_urls {
            // request again linknode_from_url -> new link node (ignore err)
            // clone curr path, add node
            // send mpsc ?
            let mut cloned_path = path.clone();
            let cloned_tx = tx.clone();


            tokio::spawn(async move {
                // add children nodes to mpsc - spawn new tasks
                let node_res = LinkNode::linknode_from_url(&child_url).await;
                if let Ok(node) = node_res {
                    cloned_path.add(node);
                    cloned_tx.send(cloned_path);

                }

                // just pattern match (goal test) here, then only add children to queue
           });

        };

        // new task - e.g up to 600-1000 per url
    
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
