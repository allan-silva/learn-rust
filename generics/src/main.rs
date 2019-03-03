fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    
    struct Point<T, U> {
        x: T,
        y: U
    }

    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y
            }
        }
    }

    impl<T> Point<T, f32> {
        fn y(&self) -> &f32 {
            &self.y
        }
    }

    fn largest<T: PartialOrd + Copy>(values: &[T]) -> T {
        let mut largest = values[0];
        for &item in values.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    #[test]
    fn largest_int() {
        let values = vec![1, 20, 3, 4];
        assert_eq!(largest(&values), 20);
    }

    fn largest_char() {
        let values = vec!['a', 'b', 'c', 'k', 'f'];
        assert_eq!(largest(&values), 'k');
    }

    #[test]
    fn generic_in_struct() {
        let integer_float = Point {x: 1u32, y: 2.3f32};
        assert_eq!(integer_float.x(), &1u32);
        assert_eq!(integer_float.y(), &2.3f32);
    }

    #[test]
    fn generic_in_struct_mixup() {
        let integer_float = Point {x: 1u32, y: 2.3f32};
        let strs = Point {x: "Hey!", y: "Teacher"};
        let the_mix = integer_float.mixup(strs);
        assert_eq!(the_mix.x, 1u32);
        assert_eq!(the_mix.y, "Teacher");
    }
}