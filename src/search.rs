use std::{collections::HashSet, fmt::Display};
use anyhow::Result;


pub struct LinkNodeData {
    pub title:String,
    child_urls:Vec<String>
}

impl LinkNodeData {
    // return a result from this
    fn get_data_from_html(html:&str) {
        // class specific to wiki (testing)
        let title_selector = scraper::Selector::parse("title").unwrap();
        let link_selector = scraper::Selector::parse("a:not(.interlanguage-link-target, .mw-jump-link)").unwrap();

        let document = scraper::Html::parse_document(&html);

        // document.title
        let title = document.select(&title_selector).into_iter().next().unwrap().inner_html();
        
        // document.querySelectorAll(a:not[...])
        let links = document.select(&link_selector);
        let links:Vec<_> = links.collect();
        println!("Links length:{}", links.len());

        // 
        let link = links.get(51).unwrap();
        let val = link.value();
        let attrs = val.attrs();
        
        for a in attrs {
            println!("attr:{:?}", a);
        }



    }
}

/// Part of a given search Path
pub struct LinkNode {
    pub url: String,
    pub data: LinkNodeData
    // pub title:String,
    // pub html: String // change to Vec<String> for Vec of child URLs
}   



impl LinkNode {
    // Result<LinkNode>
    pub async fn linknode_from_url(url:&str)->Result<()>{
        let html = reqwest::get(url)
        .await?
        .text()
        .await?;

        println!("Try getting data");
        let data = LinkNodeData::get_data_from_html(&html);

        // println!("HTML:{}", html);

        // Ok(LinkNode {
        //     url:"url".to_string(),
        //     title:"title".to_string(),
        //     html:"html".to_string()
        // })
        Ok(())
    }
}

impl Display for LinkNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.url, self.data.title)
    }
}

/// Search path along some given URLs - path list, visited set
pub struct Path {
    pub nodes: Vec<LinkNode>,
    /// use hrefs to track visited
    pub visited_hrefs: HashSet<String>
}


















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

pub fn search() {
    println!("Searching");
}