fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for v in v1_iter {
        println!("Item: {}", v);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iter_next_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn sum_demo() {
        let v1 = vec![1, 2, 3];
        assert_eq!(6, v1.iter().sum());
    }

    #[test]
    fn iter_adopters() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(vec![2, 3, 4], v2);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
        shoes
            .into_iter()
            .filter(|shoe| shoe.size == my_size)
            .collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let my_shoes = shoes_in_my_size(shoes, 10);
        assert_eq!(
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") }],
            my_shoes);
    }

    struct Counter {
        count: u32
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                count: 0
            }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_the_fucking_counter_iterator() {
        for i in Counter::new() {
            assert!(i <= 5);
        }
    }

    #[test]
    fn bizarre_test() {
        let s: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, s);
    }
}