use std::io;

fn main() {
    println!("Enter your nth number:");

    let mut digit = String::new();

    io::stdin()
        .read_line(&mut digit)
        .expect("Failed to read line");

    let digit: i32 = digit.trim().parse().expect("Please enter a valid number");

    let mut i = 0;

    let mut fibo_one = 1;
    let mut fibo_two = 1;

    while i < digit {
        println!("{fibo_one}");
        let next_fibo = fibo_one + fibo_two;
        fibo_one = fibo_two;
        fibo_two = next_fibo;
        i += 1;
    }
}
