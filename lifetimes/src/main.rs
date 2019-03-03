fn main() {
    // let str1 = String::from("Hey");
    // let result;

    // {
    //     let str2 = String::from("Teacher");
    //     result = longest(&str1, &str2);
    // }

    // println!("Longest: {}", result);

    let novel = String::from("Não maltrate os cãozinho ou eu te mato!");
    let part = novel.split(" ou ")
        .next()
        .expect("Not found");
    let important = ImportantExcerpt {part};
    println!("{}", important.announce("The part!"));
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

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        42
    }

    fn announce(&self, annoucement: &str) -> &str {
        println!("{}", annoucement);
        self.part
    }
}