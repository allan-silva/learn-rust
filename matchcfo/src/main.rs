fn main() {
    Coin::Quarter(UsState::Alabama).value_in_cents();
    Coin::Quarter(UsState::Alaska).value_in_cents();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("One quarter from {:?}", state);
                25 
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[cfg(test)]
mod tests {
    use super::{Coin, UsState};

    #[test]
    fn value_in_cents() {
        assert_eq!(Coin::Penny.value_in_cents(), 1);
        assert_eq!(Coin::Nickel.value_in_cents(), 5);
        assert_eq!(Coin::Dime.value_in_cents(), 10);
        assert_eq!(Coin::Quarter(UsState::Alabama).value_in_cents(), 25);
        assert_eq!(Coin::Quarter(UsState::Alaska).value_in_cents(), 25);
    }
}