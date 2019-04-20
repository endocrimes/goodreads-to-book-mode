#[macro_use]
extern crate serde_derive;

mod goodreads;
use chrono::prelude::*;
use chrono::Datelike;

/// A simple wrapper around writing years and year-ranges
#[derive(Debug, PartialEq, Default)]
pub struct Time {
    start: u16,
    end: u16,
}

#[derive(Debug, PartialEq, Default)]
pub struct Book {
    pub name: String,
    pub time: String,
    pub isbn: Option<String>,
    pub pages: Option<usize>,
    pub genre: Option<String>,
    pub author: Option<String>,
}

impl std::fmt::Display for Book {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.name.as_str())?;
        fmt.write_str("\t\t\t\t")?;
        fmt.write_str(self.time.as_str())?;
        if let Some(isbn) = self.isbn.clone() {
            if isbn != "" {
                fmt.write_str("\nisbn: ".into())?;
                fmt.write_str(isbn.as_str())?;
            }
        }
        Ok(())
    }
}

fn main() {
    let userid = std::env::var("USER_ID").expect("Missing env var `USER_ID`");
    let token = std::env::var("GOODREADS_TOKEN").expect("Missing env var `GOODREADS_TOKEN`");
    let response = goodreads::fetch_review_list(userid, token);
    response
        .reviews
        .reviews
        .iter()
        .map({
            |review| Book {
                name: review.book.title.clone(),
                time: format!(
                    "{}",
                    DateTime::parse_from_str(review.date_added.as_str(), "%a %b %e %T %z %Y")
                        .unwrap()
                        .year()
                ),
                isbn: review.book.isbn.clone(),
                pages: None,
                genre: None,
                author: None,
            }
        })
        .for_each(|book| println!("{}\n", book));
}
