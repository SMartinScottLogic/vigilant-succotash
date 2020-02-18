extern crate reqwest;
extern crate select;

use reqwest::Error;
use select::{document::Document, node::Node, predicate::Name};

fn link(node: Node) -> Option<(String, String)> {
    match node.attr("href") {
        Some(link) => Some((link.to_owned(), node.text())),
        None => None
    }
}

async fn fetch() -> Result<(), Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(link)
        .for_each(|x| println!("{:?}", x));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    fetch().await?;
    Ok(())
}
