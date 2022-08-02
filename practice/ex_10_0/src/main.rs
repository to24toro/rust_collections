fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // fn summarize_author(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // fn summarize_author(&self) -> String;
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more ...)")
    }

    // fn summarize_author(&self) -> String;
}


fn main() {
    let num1 = vec![34, 50, 25, 100, 65];

    let result = largest(&num1);

    println!("The largest number is {}", result);

    let num2 = vec!['y', 'm', 'a', 'q'];

    let result = largest(&num2);

    println!("The largest char is {}", result);

}