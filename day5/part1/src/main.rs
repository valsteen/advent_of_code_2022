use recap::Recap;
use serde::Deserialize;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{stdin, BufRead};

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"^move (?P<repeat>[0-9]+) from (?P<source>[0-9]+) to (?P<destination>[0-9]+)$")]
struct Move {
    repeat: usize,
    source: usize,
    destination: usize,
}

type Stacks = Vec<VecDeque<String>>;

fn parse_input() -> Result<(Stacks, Vec<Move>), Box<dyn Error>> {
    let crates_re = regex::Regex::new(r"(?P<empty>    ?)|\[(?P<label>\w)\] ?")?;
    let lanes_re = regex::Regex::new(r"(?P<lane>[0-9]+)")?;
    let lines = stdin().lock().lines().flatten().collect::<Vec<String>>();
    let mut stacks = Vec::<VecDeque<String>>::new();
    let mut idx = 0;

    while let Some(line) = lines.get(idx) {
        let mut matches = false;
        for (column, cap) in crates_re.captures_iter(line).enumerate() {
            if let Some(name) = cap.name("label") {
                matches = true;
                while column + 1 > stacks.len() {
                    stacks.push(Default::default())
                }
                stacks
                    .get_mut(column)
                    .ok_or("missing stack")?
                    .push_front(name.as_str().to_string());
            }
        }
        if !matches {
            break;
        }
        idx += 1;
    }

    if let Some(line) = lines.get(idx) {
        if lanes_re
            .captures_iter(line)
            .enumerate()
            .map_while(|(n, cap)| {
                cap.name("lane")
                    .filter(|label| (label.as_str().parse() == Ok(n + 1)))
            })
            .count()
            != stacks.len()
        {
            Err("Unexpected amount of lanes")?
        }
    } else {
        Err("Unexpected end of input")?
    }

    if lines.get(idx + 1).filter(|line| line.is_empty()).is_none() {
        Err("Expected empty line")?
    }

    let moves = lines
        .iter()
        .skip(idx + 2)
        .map(|line| line.parse())
        .collect::<Result<Vec<Move>, _>>()?;
    Ok((stacks, moves))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut stacks, moves) = parse_input()?;
    for mov in moves {
        let stack = {
            let source = stacks.get_mut(mov.source - 1).ok_or("unknown lane")?;
            if mov.repeat > source.len() {
                Err("Too many crates to move")?
            }
            source.split_off(source.len() - mov.repeat)
        };
        stacks
            .get_mut(mov.destination - 1)
            .ok_or("unknown lane")?
            .extend(stack.into_iter().rev());
    }

    let result = stacks.iter().try_fold(String::new(), |result, stack| {
        Ok::<_, Box<dyn Error>>(result + stack.back().ok_or("empty stack")?)
    })?;
    println!("{:?}", result);
    Ok(())
}
