use regex;
use std::fs::File;
use std::io::*;

struct Action {
    direction: String,
    unit: i32,
}

fn parse_file(filename: String) -> Vec<Action> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let re = regex::Regex::new(r"^([[:alpha:]]+) (\d+)$").unwrap();

    let mut result: Vec<Action> = Vec::new();

    for line in reader.lines() {
        let value = line.unwrap();
        let caps = re.captures(&value).unwrap();

        result.push(Action {
            direction: caps
                .get(1)
                .map_or("".to_string(), |m| String::from(m.as_str())),
            unit: caps.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
        });
    }

    return result;
}

fn part_1(instructions: &Vec<Action>) {
    let (mut depth, mut position) = (0, 0);

    for instruction in instructions {
        match instruction.direction.as_str() {
            "forward" => position += instruction.unit,
            "down" => depth += instruction.unit,
            "up" => depth -= instruction.unit,
            _ => println!("unkown command: {}", instruction.direction),
        }
    }

    println!(
        "Answer: {} (depth: {}, position: {})",
        depth * position,
        depth,
        position
    );
}

fn part_2(instructions: &Vec<Action>) {
    let (mut aim, mut depth, mut position) = (0, 0, 0);

    for instruction in instructions {
        match instruction.direction.as_str() {
            "forward" => {
                position += instruction.unit;
                depth += aim * instruction.unit
            }
            "down" => aim += instruction.unit,
            "up" => aim -= instruction.unit,
            _ => println!("unkown command: {}", instruction.direction),
        }
    }

    println!(
        "Answer: {} (depth: {}, position: {}, aim: {})",
        depth * position,
        depth,
        position,
        aim
    );
}

pub fn run() {
    let filename: String = "data/example_day_2".to_string();
    let filename: String = "data/day2".to_string();
    let instructions = parse_file(filename);

    part_1(&instructions);
    part_2(&instructions);
}
