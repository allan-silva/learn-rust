fn main() {
    let mut s = String::from("Hello world");
    let fw = first_word(&s);

    // s.clear();

    println!("First: {}", fw);

    let s2 = "Hey teacher";
    let fw = first_word(s2);
    println!("First: {}", fw);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
