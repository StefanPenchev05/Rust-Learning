pub struct Author {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Author {
    pub fn new(first_name: String, last_name: String, age: i32) -> Self {
        Self {
            first_name,
            last_name,
            age,
        }
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: Author,
    pub content: String,
}

impl NewsArticle {
    pub fn new(headline: String, location: String, author: Author, content: String) -> Self {
        Self {
            headline,
            location,
            author,
            content,
        }
    }
}

pub trait Summary {
    fn summnary(&self) -> String{
        format!("(Read more from {}...)", self.summary_author())
    }
    fn summary_author(&self) -> String;
}

impl Summary for NewsArticle {
    fn summnary(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline, self.author.first_name, self.location
        )
    }

    fn summary_author(&self) -> String {
        format!(
            "Author {} {}, age: {}",
            self.author.first_name, self.author.last_name, self.author.age
        )
    }
}

pub fn notify(item: &impl Summary){
    println!("Breaking new! {}", item.summnary());
}
