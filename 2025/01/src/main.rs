use std::{env, error::Error, fs};

fn parse_input(input_string: &str) -> Vec<&str> {
    input_string.lines().collect()
}

fn cost(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let bytes = line.bytes().len() <= 160;
            let chars = line.chars().count() <= 140;

            if bytes && chars {
                return 13;
            } else if bytes {
                return 11;
            } else if chars {
                return 7;
            }

            0
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let lines = parse_input(&contents);

    println!("{:?}", cost(&lines));

    Ok(())
}
