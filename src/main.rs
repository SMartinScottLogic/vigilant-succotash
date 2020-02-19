extern crate reqwest;
extern crate select;
extern crate url;
extern crate magic;

//use reqwest::Error;
use select::{document::Document, node::Node, predicate::Name};
use url::{Url, Position, ParseOptions};
use magic::{Cookie, CookieFlags};

fn get_base_url(url: &Url, doc: &Document) -> Result<Url, url::ParseError> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url = base_tag_href.map_or_else(
        || Url::parse(&url[..Position::BeforePath]),
        Url::parse,
    )?;

    Ok(base_url)
}

fn link(base_url: &ParseOptions, node: Node) -> Option<(String, String)> {
    node.attr("href").map(|link| {
        match base_url.parse(link) {
            Ok(rlink) => Some((rlink.to_string(), node.text())),
            Err(_) => None
        }
    }).flatten()
}

async fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;

    let body = reqwest::get(url.as_ref())
        .await?
        .text()
        .await?;
    let document = Document::from(body.as_str());
    let base_url = get_base_url(&url, &document)?;
    let base_parser = Url::options().base_url(Some(&base_url));

    println!("{:?}", Cookie::open(CookieFlags::default()).unwrap().buffer(body.as_bytes()));

    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|node| link(&base_parser, node))
        .for_each(|x| println!("{:?}", x));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch().await?;
    Ok(())
}
