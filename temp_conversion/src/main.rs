use std::io;

fn main() {
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Input error.");
    
    let temp: f32 = temp
        .trim()
        .parse()
        .expect("Please type a number.");
    
    println!("Temperature in Fahrenheit: {temp}.");
    
    let temp: f32 = (temp - 32.0)/1.8;
    
    println!("Temperature in Celsius: {temp}.");
}