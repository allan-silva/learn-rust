use std::ops::Deref;

use crate::List::{Cons, Nil};


fn main() {
    let b = Box::new(5);
    println!("Int in the heap: {}", b);

    let list = Cons(42,
        Box::new(
            Cons(21,
                Box::new(
                    Cons(21,
                        Box::new(Nil))))));

    let m = 5;
    let y = MyBox::new(m);

    assert_eq!(5, m);
    assert_eq!(5, *y);

    let hello = MyBox::new(String::from("Rust"));
    say_hello(&hello);
}


enum List {
    Cons(i32, Box<List>),
    Nil
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}


fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}