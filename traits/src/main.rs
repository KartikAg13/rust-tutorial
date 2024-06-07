use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author) 
    }
}

pub struct  Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }


    //default implemantation is overriden
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) 
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    //default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//item can be anything that implements summary
//syntax of trait bounds
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//we can also use this syntax instead, (remember generics)
/* 
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
*/

//another syntax for trait bounds
fn some_function<T, U>(_t: &T, _u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug + Summary
{
    1
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you already know people"),
        reply: false,
        retweet: false
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("KartikAg13"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("Kartik Agrawal"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}