struct Point<T> { x: T, y: T }
// struct Line<T,U> { x: T, y: U }
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub _content: String,
}
impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub _reply: bool,
    pub _repost: bool,
}
impl Summary for SocialPost {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub trait Summary {
    fn summarise(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result2 = largest(&number_list_2);
    println!("The largest number is {result2}");

    let integer = Point { x: 5, y: 10 };
    //let _float = Point { x: 1.0, y: 4.0 };
    //let _new_line = Line { x: "Hello", y: 'c' };
    println!("integer X = {}, integer Y = {}", integer.x(), integer.y());

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        _reply: false,
        _repost: false,
    };
    let article = NewsArticle {
        headline: String::from("Poopy"),
        location: String::from("Poopy"),
        _content: String::from("Poopy"),
        author: String::from("horse_ebooks"),
    };

    println!("1 new post: {}", post.summarise());
    println!("1 new article: {}", article.summarise());
}
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn _notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarise());
}