use std::{env, error::Error, fs};

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn is_valid(password: &str) -> bool {
    let char_len = password.chars().count();
    if !(4..=12).contains(&char_len) {
        return false;
    }

    let mut digit = false;
    let mut uppercase = false;
    let mut lowercase = false;
    let mut non_ascii = false;

    for c in password.chars() {
        if c.is_numeric() {
            digit = true;
        }

        if c.is_uppercase() {
            uppercase = true;
        }

        if c.is_lowercase() {
            lowercase = true;
        }

        if !c.is_ascii() {
            non_ascii = true;
        }
    }

    digit && uppercase && lowercase && non_ascii
}

fn check_passwords(passwords: &[&str]) -> usize {
    passwords.iter().fold(0, |acc, &p| {
        if is_valid(p) {
            return acc + 1;
        }

        acc
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let passwords = parse_input(&contents);

    println!("{}", check_passwords(&passwords));

    Ok(())
}
