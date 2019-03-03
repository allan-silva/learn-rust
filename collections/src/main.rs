fn main() {
    let mut i = 41;
    let test_enum = Teste::OK(& mut i);
    println!("{:?}", test_enum);
    let mut teste = test_enum.get_ref_mut();
    *teste += 1;
    println!("{}", i);
    println!("{:?}", test_enum);
}

#[derive(Debug)]
enum Teste<'a> {
    OK(&'a mut i32),
    None
}

impl<'a> Teste<'a> {
    fn get_ref_mut(self) -> &'a mut i32 {
        match self {
            Teste::OK(value) => value,
            _ => panic!("Panic!")
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn new_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);
        assert_eq!(scores.get("blue").unwrap(), &10);
        assert_eq!(scores.get("yellow").unwrap(), &50);
    }

    #[test]
    fn new_hashmap_from_tuples() {
        let teams = vec![String::from("blue"), String::from("yellow")];
        let scores_v = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(scores_v.iter()).collect();
        let k1 = String::from("blue");
        let k2 = String::from("yellow");
        assert_eq!(**scores.get(&k1).unwrap(), 10);
        assert_eq!(**scores.get(&k2).unwrap(), 50);
    }

    #[test]
    fn new_vec() {
        let new_v : Vec<i32> = Vec::new();
        assert_eq!(new_v.len(), 0);
    }

    #[test]
    fn new_vec_with_values() {
        let new_v = vec![1, 2, 3];
        assert_eq!(new_v.len(), 3);
    }

    #[test]
    fn push_item() {
        let mut new_v = Vec::new();
        new_v.push(1);
        new_v.push(2);
        assert_eq!(new_v.len(), 2);
    }

    #[test]
    fn read_items_from_index() {
        let new_v = vec![1, 2, 3];
        for i in 0..new_v.len() {
            let item: &i32 = &new_v[i];
            assert!(*item == 1 || *item == 2 || *item == 3);
        }
    }

    #[test]
    fn read_items_from_enumeration() {
        let new_v = vec![7, 8, 9];
        for i in &new_v {
            assert!(*i == 7 || *i == 8 || *i == 9);
        }
    }

    #[test]
    fn match_index() {
        let new_v = vec![42, 4242];
        let item = match new_v.get(0) {
            Some(i) => i,
            None => &-1
        };
        assert_eq!(*item, 42);

        let item = match new_v.get(1) {
            Some(i) => i,
            None => &-1
        };
        assert_eq!(*item, 4242);

        let item = match new_v.get(2) {
            Some(i) => i,
            None => &-1
        };
        assert_eq!(*item, -1);
    }

    #[test]
    fn change_values() {
        let mut new_v = vec![42, 4242];
        for i in &mut new_v {
            *i *= 2;
        }

        for i in &new_v {
            assert!(*i == 42 * 2 || *i == 4242 * 2);
        }
    }

    #[test]
    fn new_string() {
        let s = String::new();
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn new_from_str() {
        let s = "I have content!".to_string();
        assert_eq!(s, "I have content!");
    }

    #[test]
    fn concat_keep_ownership() {
        let mut s = String::from("Hey, ");
        s.push_str("teacher,");
        let s2 = " leave kids alone!";
        s.push_str(s2);
        assert_eq!(s, "Hey, teacher, leave kids alone!");
        assert_eq!(s2, " leave kids alone!");
    }

    #[test]
    fn concat_loosing_ownership() {
        let s = String::from("Hey, ");
        let s2 = String::from("teacher,");
        let s3 = String::from(" leave kids alone!");
        let result_str = s + &s2 + &s3;
        assert_eq!(result_str, "Hey, teacher, leave kids alone!");
        // Reference is not valid anymore, due add(Self, &T) signature.
        //assert_eq!(s, "Hey, ");
    }

    #[test]
    fn format_str() {
        let s = String::from("Hey, ");
        let s2 = String::from("teacher,");
        let s3 = String::from(" leave kids alone!");
        let result_str = format!("{}{}{}", s, s2, s3);
        assert_eq!(result_str, "Hey, teacher, leave kids alone!");
        assert_eq!(s, "Hey, ");
    }
}