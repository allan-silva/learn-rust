fn main() {
    let x = 5;
    println!("Value of x: {}", x);

    {
        let x = x * 2;
        println!("Value of x: {}", x);
    }

    println!("Value of x: {}", x);    
}
