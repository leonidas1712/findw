use url::{Url};

pub fn get_base_url(parsed_url:Url) {

}

pub fn debug_url(url:&str) {
    let parsed = Url::parse(url).unwrap();
    
    println!("Domain: {:?}", parsed.domain());
    println!("Base: {:?}", parsed.scheme());
    println!("Port:{:?}", parsed.port());
    println!("");
}