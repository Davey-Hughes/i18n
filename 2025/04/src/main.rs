use std::{env, error::Error, fs};

use chrono::NaiveDateTime;
use chrono_tz::Tz;
use regex::Regex;

fn sum_times(input: &str) -> Result<i64, Box<dyn Error>> {
    let re = Regex::new(r"([a-zA-Z]+:\s*)(\S+[/\S+]+)\s*(.*)")?;

    let mut lines = vec![];
    for (_, [_, tz, time]) in re.captures_iter(input).map(|c| c.extract()) {
        let dt = NaiveDateTime::parse_from_str(time, "%b %d, %Y, %H:%M")?;
        lines.push(dt.and_local_timezone(tz.parse::<Tz>()?).unwrap());
    }

    Ok(lines.chunks(2).map(|c| (c[1] - c[0]).num_minutes()).sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    println!("{}", sum_times(&contents)?);

    Ok(())
}
