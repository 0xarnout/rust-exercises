use std::io;


fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Reading input failed.");
    
    let n: u8 = n
        .trim()
        .parse()
        .expect("Please enter a positive number");
    
    let result = fibonacci(n);

    println!("the {n}th fibonacci number is: {result}");
}

fn fibonacci(n: u8) -> u128 {
    let mut num1: u128 = 0;
    let mut num2: u128 = 1;
    for _ in 1..n {
        num1 += num2;
        num2 ^= num1;
        num1 ^= num2;
        num2 ^= num1;
    }
    num2
}