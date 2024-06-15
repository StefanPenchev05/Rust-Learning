use std::{fmt::{Display, Result}, result};

struct ImportExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportExcerpt<'a> {
    fn _level(&self) -> i32 {
        32
    }

    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "abcd";
    let s2 = "abc";
    let result = longest(&s1, &s2);
    println!("{}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentance = novel.split('.').next().expect("Could not find .");
    let _i = ImportExcerpt {
        part: &first_sentance,
    };

    let result = longest_with_an_announcement(&s1, s2 , "test announcement");
    println!("{result}");
}
