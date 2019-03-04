use std::thread;
use std::time::Duration;


struct Cacher<T>
    where T: Fn() -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn() -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)();
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn take_ownership(i: Vec<u32>) {}

fn generate_workout(intensity: u32, random_number: u32) {
    let intensities = vec![intensity];

    let mut expensive_result = Cacher::new(move || {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensities[0]
    });

    //take_ownership(intensities); //uncomment to get move error

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value()
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value()
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value()
            );
        }
    }
}
