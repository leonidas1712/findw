use url::{Url};
use anyhow::{anyhow, Result};

/// Get base e.g http://localhost:8000/index.html => http://localhost:8000
pub fn get_base_url(url:&str)->Result<Url> {
    let parsed = Url::parse(url);
    if parsed.is_err() {
        return Err(anyhow!("Invalid url:{}", url))
    }
    
    let parsed = parsed.unwrap();
    let domain = parsed.domain();

    if domain.is_none() {
        return Err(anyhow!("URL '{}' has no domain.", url));
    }

    let scheme = parsed.scheme(); // http or https
    let domain = domain.unwrap(); // localhost or blog.janestreet.com
    let port = parsed.port(); // 8000

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
    Ok(base_url)
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
    use super::get_base_url;

    #[test]
    pub fn test_get_base_url() {
        let local = "http://localhost:8000/index.html";
        let res = get_base_url(local);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!("http://localhost:8000/", res.to_string());


        let norm = "https://blog.janestreet.com/what-the-interns-have-wrought-2023/";
        let res = get_base_url(norm);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!("https://blog.janestreet.com/", res.to_string());
    }
}