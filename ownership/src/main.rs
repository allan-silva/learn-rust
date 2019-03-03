fn main() {
    let s1 = String::from("This fucking string is on HEAP!");
    let i_am_a_f_clone = s1.clone();

    println!("S1: {}, S2: {}", s1, i_am_a_f_clone);

    let x = 5;
    let y = x;

    println!("No deep copy here man! {} & {}", x, y);

    let s1 = gives_ownership();
    let s2 = String::from("Ho!");
    let s3 = take_o_give_back(s2);

    let (s4, l) = calc_length(s3);
    println!("S: {}, L: {}", s4, l);

    let l2 = calc_len_by_ref(&s4);
    println!("S: {}, L: {}", s4, l2);

    let mut s5 = String::from("Hey, Ho!");
    change_ref(&mut s5);
    println!(">>>>{}<<<<", s5);
}

fn gives_ownership() -> String {
    let s = String::from("Hey!");
    s
}

fn take_o_give_back(s: String) -> String {
    s
}

fn calc_length(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn calc_len_by_ref(s: &String) -> usize {
    s.len()
}

fn change_ref(s: &mut String) {
    s.push_str(", letisgou da silva");
}