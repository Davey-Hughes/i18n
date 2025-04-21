use std::{collections::HashSet, env, error::Error, fs};

#[derive(Debug)]
struct PuzzleLine {
    c: char,
    pos: usize,
    len: usize,
}

fn fix_encoding(s: &str) -> String {
    let bytes = s.bytes().enumerate().collect::<Vec<(usize, u8)>>();
    let pos = bytes.windows(3).fold(HashSet::new(), |mut acc, w| {
        if w[0].1 == 0xc3 && (w[2].1 == 0xc2 || w[2].1 == 0xc3) {
            acc.insert(w[0].0 + 1);
            acc.insert(w[0].0 + 2);
        }
        acc
    });

    let bytes = bytes.into_iter().fold(vec![], |mut acc, (i, x)| {
        if !pos.contains(&i) {
            acc.push(x);
        }
        acc
    });

    String::from_utf8(bytes).unwrap()
}

fn parse_input(contents: &str) -> Result<(Vec<String>, Vec<PuzzleLine>), Box<dyn Error>> {
    let (dict, puzzle) = contents
        .split_once("\n\n")
        .ok_or("Malformed input. Expected blank line between dictionary and puzzle.")?;

    let dict = dict
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let mut res = s.to_string();

            if i % 3 == 2 {
                res = fix_encoding(&res);
            }

            if i % 5 == 4 {
                res = fix_encoding(&res);
            }

            res
        })
        .collect::<Vec<String>>();

    let puzzle = puzzle
        .lines()
        .map(str::trim)
        .map(|s| {
            let pos = s.chars().position(|c| c != '.').unwrap();
            PuzzleLine {
                c: s.chars().nth(pos).unwrap(),
                pos,
                len: s.len(),
            }
        })
        .collect::<Vec<_>>();

    Ok((dict, puzzle))
}

fn match_word(dict: &[String], line: &PuzzleLine) -> Option<usize> {
    for (i, s) in dict.iter().enumerate() {
        if s.chars().count() == line.len && s.chars().nth(line.pos) == Some(line.c) {
            return Some(i + 1);
        }
    }

    None
}

fn solve(dict: &[String], puzzle: &[PuzzleLine]) -> usize {
    puzzle.iter().map(|l| match_word(dict, l).unwrap()).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(env::args().nth(1).expect("Input file expected as argument"))?;

    let (dict, puzzle) = parse_input(&contents)?;

    println!("{}", solve(&dict, &puzzle));

    Ok(())
}
