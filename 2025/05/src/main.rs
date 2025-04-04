use std::{env, error::Error, fs};

fn parse_input(input_string: &str) -> Vec<Vec<char>> {
    input_string.lines().map(|l| l.chars().collect()).collect()
}

fn count_poop(input: &[Vec<char>]) -> usize {
    let mut x = 0;

    let mut poop = 0;

    for row in input {
        if row[x] == '💩' {
            poop += 1;
        }

        x = (x + 2) % input[0].len();
    }

    poop
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let input = parse_input(&contents);

    println!("{}", count_poop(&input));

    Ok(())
}
