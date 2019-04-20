use reqwest;
use serde_xml_rs::from_str;


#[derive(Debug, PartialEq, Deserialize)]
pub struct ReviewListResponse {
    pub reviews: ReviewList,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ReviewList {
    pub start: i32,
    pub end: i32,
    pub total: i32,

    #[serde(rename = "$value")]
    pub reviews: Vec<BookReview>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct BookReview {
    pub id: i64,
    pub book: Book,
    pub date_added: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Book {
    pub id: i64,
    pub isbn: Option<String>,
    pub title: String,
}

pub fn fetch_review_list(userid: String, token: String) -> ReviewListResponse {
    // TODO: Paginate for people with more than 200 books in goodreads
    let uri = format!(
        "https://www.goodreads.com/review/list/{}.xml?key={}&v=2&per_page=200",
        userid, token
    );
    let client = reqwest::Client::new();
    let mut response = client
        .get(&uri)
        .send()
        .expect("Failed to fetch goodreads list");

    let text = response.text().expect("Failed to read response body");
    from_str(text.as_str()).expect("Failed to decode goodreads response")
}

#[test]
fn decode_reviews() {
    let response = r#"<reviews start="1" end="20" total="65">
            <review>
                <id>2649705306</id>
                <date_added>2018</date_added>
                <book>
                    <id>1</id>
                    <title>Test</title>
                </book>
            </review>
        </reviews>"#;

    let expected = ReviewList {
        total: 65,
        start: 1,
        end: 20,
        reviews: vec![BookReview {
            id: 2649705306,
            date_added: "2018".into(),
            book: Book {
                id: 1,
                isbn: None,
                title: "Test".into(),
            },
        }],
    };
    let result: ReviewList = from_str(response).expect("ugh");
    assert_eq!(result, expected);
}

#[test]
fn decode_full_response() {
    let response = r#"<GoodreadsResponse>
        <Request></Request>
        <reviews start="1" end="20" total="65">
            <review>
                <id>2649705306</id>
                <date_added>2018</date_added>
                <book>
                    <id>1</id>
                    <isbn>123141</isbn>
                    <title>Test</title>
                </book>
            </review>
        </reviews>
        </GoodreadsResponse>"#;

    let expected = ReviewListResponse {
        reviews: ReviewList {
            total: 65,
            start: 1,
            end: 20,
            reviews: vec![BookReview {
                id: 2649705306,
                date_added: "2018".into(),
                book: Book {
                    id: 1,
                    isbn: Some("123141".into()),
                    title: "Test".into(),
                },
            }],
        },
    };
    let result: ReviewListResponse = from_str(response).expect("ugh");
    assert_eq!(result, expected);
}

#[test]
fn decode_real_response() {
    let response = include_str!("example.xml");
    let _: ReviewListResponse = from_str(response).expect("ugh");
}
