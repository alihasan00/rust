use std::{
    fs::File,
    io::{self, Write},
};

enum Format {
    Simple,
    Vertical,
    Emoji,
}

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

fn read_format(prompt: &str) -> io::Result<Format> {
    println!("{prompt}");
    println!("Choices: 1. Simple 2. Vertical 3. Emoji");

    loop {
        let input = read_input("")?;
        match input.as_str() {
            "1" => return Ok(Format::Simple),
            "2" => return Ok(Format::Vertical),
            "3" => return Ok(Format::Emoji),
            _ => println!("please enter a correct choice 1. Simple 2. Vertical 3. Emoji"),
        }
    }
}

fn format_profile(
    format: Format,
    name: &str,
    age: i32,
    city: &str,
    profession: &str,
    hobby: &str,
    website: &str,
    emoji: &str,
) -> String {
    match format {
        Format::Simple => {
            let mut result = format!(
                "Name: {}\nAge: {}\nCity: {}\nProfession: {}",
                name, age, city, profession
            );
            if !hobby.is_empty() {
                result.push_str(&format!("\nHobby: {}", hobby));
            }
            if !website.is_empty() {
                result.push_str(&format!("\nWebsite: {}", website));
            }
            result
        }
        Format::Vertical => {
            let mut result = format!(
                "------------\n| {} |\n| {} |\n| {} |\n| {} |",
                name, age, city, profession
            );
            if !hobby.is_empty() {
                result.push_str(&format!("\n| {} |", hobby));
            }
            if !website.is_empty() {
                result.push_str(&format!("\n| {} |", website));
            }
            result.push_str("\n------------");
            result
        }
        Format::Emoji => {
            let mut result = format!(
                "{}\nName: {}\nAge: {}\nCity: {}\nProfession: {}",
                emoji, name, age, city, profession
            );
            if !hobby.is_empty() {
                result.push_str(&format!("\nHobby: {}", hobby));
            }
            if !website.is_empty() {
                result.push_str(&format!("\nWebsite: {}", website));
            }
            result.push_str(&format!("\n{}", emoji));
            result
        }
    }
}

fn main() {
    let name = read_input("Please enter your name: ").expect("Failed to read name");

    let age = read_age("Please enter your age: ").expect("Failed to read age");

    let city = read_input("Please enter your city: ").expect("Failed to read city");

    let profession =
        read_input("Please enter your profession: ").expect("Failed to read profession");

    let hobby = read_input("Please enter one line passion, goal or favorite emoji (optional). ")
        .expect("Failed to read hobby");

    let website = read_input("Please enter a website/social media handle (optional). ")
        .expect("failed to read website");

    let format = read_format("Please select the format type").expect("failed to ge the format");

    let emoji = read_input("Please enter an emoji: ").expect("failed to read emoji");

    let formatted = format_profile(
        format,
        name.as_str(),
        age,
        city.as_str(),
        profession.as_str(),
        hobby.as_str(),
        website.as_str(),
        emoji.as_str(),
    );
    println!("{}", formatted);
    let filename = format!("{}.txt", name);
    let mut file = File::create(filename).expect("failed to create file");
    file.write_all(formatted.as_bytes())
        .expect("failed to write to file");
}
