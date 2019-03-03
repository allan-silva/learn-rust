fn main() {
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height:  size
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rectangle};

    #[test]
    fn test_area() {
        let r = Rectangle {width: 3., height: 4.};
        assert_eq!(r.area(), 12_f64);
    }

    #[test]
    fn test_square() {
        let s = Rectangle::square(42.);
        assert_eq!(s.height, s.width);
        assert_eq!(s.height, 42.);
        assert_eq!(s.area(), 42. * 42.);
    }
}