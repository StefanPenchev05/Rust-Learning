mod lib;
use lib::{notify, Author, NewsArticle, Summary};



fn main() {
    let author = Author::new("Stefan".to_string(), "Penchev".to_string(), 18);
    let article = NewsArticle::new(
        "Life in Pazardjik".to_string(),
        "Pazardjik, Bulgaria".to_string(),
        author,
        "Some random text, just for testing".to_string(),
    );

    println!("{}", article.summnary());
    println!("{}", article.summary_author());
    notify(&article);
}
