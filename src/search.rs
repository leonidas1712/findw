use crate::DynResult;

/// Request URL and get HTML string
pub fn request_link(url:&str) -> DynResult<String> {

    let res = String::from("Requested");
    Ok(res)
}

pub fn search() {
    println!("Searching");
}