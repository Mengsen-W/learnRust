#![allow(unused)]
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
    fn summarize_other(&self) -> String {
        self.summarize()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// other function likes summarize_default use default
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl<T> Summary for Vec<T> {
    fn summarize(&self) -> String {
        return format!("vec length = {}", self.len());
    }
}

pub fn notify_1(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyr_one<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2(item1: impl Summary, item2: impl Summary) {
    println!(
        "Breaking news! {} + {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_two<T: Summary>(item1: T, item2: T) {
    println!(
        "Breaking news! {} + {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_d(item: impl Summary + std::fmt::Display + std::fmt::Debug) {
    println!("{} + {:?}", item.summarize(), item)
}
pub fn notify_disp<T: Summary + std::fmt::Display + std::fmt::Debug>(item: T) {
    println!("{} + {:?}", item.summarize(), item)
}

pub fn some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(
    _t: T,
    _u: U,
) -> i32 {
    0
}
pub fn some_function_where<T, U>(_t: T, _u: U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}

// do not Returns in the function two types
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }

    // errror
    // if return xxx;
    // else return yyy;
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + std::cmp::PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: &'a T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
