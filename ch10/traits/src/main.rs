use std::fmt::Display;

pub trait Summary {
    fn view_count(&self) -> i16 {
        0
    }
    fn summarize(&self) -> String {
        format!("Read more about this story with {} viwes", self.view_count())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}), {} views", self.headline, self.author, self.location, self.view_count())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct RedditPost {

    pub username: String,
    pub updoots: i16,
    pub content: String,
    pub views: i16,
}

impl Summary for RedditPost {
    fn view_count(&self) -> i16 {
        self.views
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// disgusting function (to my newbie eyes)
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best team in the world"),
    };
    println!("New article available! {}", article.summarize());
    let redditpost = RedditPost {
        username: String::from("horse_ebooks"),
        updoots: 12,
        content: String::from("of course, as you probably already know, people"),
        views: 100,
    };
    println!("1 new reddit post: {}", redditpost.summarize());
    notify(&article);
    notify_trait_bound(&article);
    let strone = "long string";
    let strtwo = "longer string";
    println!("The longest string is {}", longest(strone, strtwo));

    let string1 = String::from("long string is long");
    let result: &str;
    {
        let string2 = String::from("short string");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    let string2 = String::from("short string");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "hello");
}
