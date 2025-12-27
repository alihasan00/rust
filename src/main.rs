use std::io::{self, Write};

fn read_input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_age(prompt: &str) -> io::Result<i32> {
    loop {
        let input = read_input(prompt)?;
        match input.parse::<i32>() {
            Ok(age) if age > 0 => return Ok(age),
            Ok(_) => println!("Please enter a valid positive age."),
            Err(_) => println!("Please enter a valid number for age."),
        }
    }
}

fn main() {
    let name = read_input("Please enter your name: ").expect("Failed to read name");

    let age = read_age("Please enter your age: ").expect("Failed to read age");

    let city = read_input("Please enter your city: ").expect("Failed to read city");

    let profession =
        read_input("Please enter your profession: ").expect("Failed to read profession");

    let hobby = read_input("Please enter your hobby: ").expect("Failed to read hobby");

    println!(
        "Hello, my name is {name}, I am {age} years old and live in {city}, and I work as a {profession} and absolutely enjoy {hobby} in my free time. Nice to meet you!"
    );
}
