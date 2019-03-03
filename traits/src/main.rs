use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("X = {} maior ou igual a Y", self.x);
        } else {
            println!("Y = {} maior que X", self.y);
        }
    }
}

fn main() {
    let integers = Pair {x: 1, y: 2};
    integers.cmp_display();

    // let pairs = Pair {x: &integers, y: &integers};
    // pairs.cmp_display();
}

mod aggregator {
    pub trait Summary {
        fn summarize(&self) -> String{
            String::from(" read more...")
        }
    }

    pub trait SummaryDefaultDisplay {
        fn deafult_text(&self) -> String;
    }

    //Blanket implementation
    impl<T: Summary> SummaryDefaultDisplay for T {
        fn deafult_text(&self) -> String {
            String::from("Default summary content")
        }
    }

    pub struct BlogArticle {
    }

    impl Summary for BlogArticle {}

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
}

#[cfg(test)]
mod tests {
    use crate::aggregator::{NewsArticle, Tweet, BlogArticle, Summary, SummaryDefaultDisplay};
    
    pub fn summarize(item: &impl Summary) -> String {
        item.summarize()
    }

    pub fn summarize_2<T: Summary>(item: &T) -> String {
        summarize(item)
    }

    #[test]
    fn test_summary() {
        let tweet = Tweet {
            username: String::from("Allan"),
            content: String::from("Hey teacher!"),
            reply: true,
            retweet: false,
        };

        let article = NewsArticle {
            headline: String::from("Hey teacher!"),
            author: String::from("Allan"),
            location: String::from("Sorocaba"),
            content: String::from("Hey teacher leave kids alone!")
        };

        let blog_entry = BlogArticle {};

        assert_eq!(tweet.summarize(), "Allan: Hey teacher!");
        assert_eq!(article.summarize(), "Hey teacher!, by Allan (Sorocaba)");
        assert_eq!(blog_entry.summarize(), " read more...");

        assert_eq!(summarize(&tweet), "Allan: Hey teacher!");
        assert_eq!(summarize(&article), "Hey teacher!, by Allan (Sorocaba)");
        assert_eq!(summarize(&blog_entry), " read more...");

        assert_eq!(summarize_2(&tweet), "Allan: Hey teacher!");
        assert_eq!(summarize_2(&article), "Hey teacher!, by Allan (Sorocaba)");
        assert_eq!(summarize_2(&blog_entry), " read more...");

        assert_eq!(blog_entry.deafult_text(), "Default summary content");
    }
}