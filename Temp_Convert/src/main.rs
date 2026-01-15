use std::io;

fn main() {
    println!("Enter Temp In Celsius : ");
    let mut celsius = String :: new();
    io::stdin().read_line(&mut celsius).expect("Not able to get input");
    let celsius :f64  = celsius.trim().parse().expect("Not able to convert celsius into float");
    let farenheit :f64= (celsius * 9.0/5.0) +32.0;
    println!("Temperature in farenheit is {farenheit}");
}
