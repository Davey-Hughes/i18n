use std::{collections::HashMap, env, error::Error, fs};

use chrono::{DateTime, Utc};

fn parse_input(contents: &str) -> Vec<DateTime<Utc>> {
    contents.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_event(lines: &[DateTime<Utc>]) -> Option<DateTime<Utc>> {
    let mut map: HashMap<DateTime<Utc>, usize> = HashMap::new();

    for line in lines {
        let entry = map.entry(*line).and_modify(|e| *e += 1).or_default();
        if *entry == 3 {
            return Some(*line);
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let lines = parse_input(&contents);

    println!(
        "{}",
        find_event(&lines).ok_or("No time found")?.to_rfc3339()
    );

    Ok(())
}
