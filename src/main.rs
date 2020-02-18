extern crate reqwest;
extern crate select;

use reqwest::Error;
use select::document::Document;
use select::predicate::Name;

async fn fetch() -> Result<(), Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    fetch().await?;
    Ok(())
}
