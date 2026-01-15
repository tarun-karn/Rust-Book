use std::io;

fn fibonacci_number(n: i64) -> i64 {
    if n < 0 {
        -1
    } else if n == 1 {
        1
    } else if n == 0 {
        0
    } else {
        fibonacci_number(n - 1) + fibonacci_number(n - 2)
    }
}

fn main() {
    println!("Enter N to find the Fibonacci number:");

    let mut num: String = String::new();
    io::stdin().read_line(&mut num).expect("Failed to take input");

    let num: i64 = num.trim().parse().expect("Unable to convert to int");

    let result = fibonacci_number(num);

    if num > 0 {
        println!("Fibonacci number is {result}");
    }
}
