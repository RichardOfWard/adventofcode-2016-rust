extern crate core;

use std::fs::File;
use std::iter::Iterator;
use std::io::{Read};
use self::core::str::FromStr;

#[derive(Clone, Copy)]
enum TaxiRotation {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum TaxiOrientation {
    North,
    East,
    South,
    West,
}

type Position = (i64, i64);

type Instruction = (TaxiRotation, i64);

impl FromStr for TaxiRotation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(TaxiRotation::Left),
            "R" => Ok(TaxiRotation::Right),
            _ => Err("Unknown relative direction ".to_string() + s),
        }
    }
}

impl TaxiOrientation {
    fn rotate(self, rel: TaxiRotation) -> TaxiOrientation {
        match rel {
            TaxiRotation::Right => match self {
                TaxiOrientation::North => TaxiOrientation::East,
                TaxiOrientation::East => TaxiOrientation::South,
                TaxiOrientation::South => TaxiOrientation::West,
                TaxiOrientation::West => TaxiOrientation::North,
            },
            TaxiRotation::Left => match self {
                TaxiOrientation::East => TaxiOrientation::North,
                TaxiOrientation::South => TaxiOrientation::East,
                TaxiOrientation::West => TaxiOrientation::South,
                TaxiOrientation::North => TaxiOrientation::West,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Taxi {
    orientation: TaxiOrientation,
    position: Position,
}

impl Taxi {
    fn rotate(self, direction: TaxiRotation) -> Taxi {
        Taxi {
            orientation: self.orientation.rotate(direction),
            position: self.position,
        }
    }
    fn proceed(self) -> Taxi {
        Taxi {
            orientation: self.orientation,
            position: match self.orientation {
                TaxiOrientation::North => (self.position.0 + 1, self.position.1),
                TaxiOrientation::South => (self.position.0 - 1, self.position.1),
                TaxiOrientation::East => (self.position.0, self.position.1 + 1),
                TaxiOrientation::West => (self.position.0, self.position.1 - 1),
            },
        }
    }
}

fn generate_taxi_history(instructions: Vec<Instruction>) -> Vec<Taxi> {
    let mut taxi = Taxi {
        position: (0, 0),
        orientation: TaxiOrientation::North,
    };
    let mut history = vec!(taxi);
    for instruction in instructions {
        taxi = taxi.rotate(instruction.0);
        for _ in 0..instruction.1 {
            taxi = taxi.proceed();
            history.push(taxi);
        }
    }
    history
}

fn taxi_abs(position: Position) -> i64 {
    (position.0 + position.1).abs()
}

fn load_instructions() -> Vec<Instruction> {
    read_file("inputs/1-1.txt")
        .trim()
        .split(", ")
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(str: &str) -> Instruction {
    (str[0..1].parse::<TaxiRotation>().unwrap(), str[1..].parse::<i64>().unwrap())
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}

fn part_1(instructions: Vec<Instruction>) -> i64 {
    taxi_abs(generate_taxi_history(instructions).last().unwrap().position)
}

#[test]
fn taxicab_abs_solves_the_part_1_examples() {
    assert_eq!(5, part_1(vec!(
        (TaxiRotation::Right, 2),
        (TaxiRotation::Left, 3),
    )));

    assert_eq!(2, part_1(vec!(
        (TaxiRotation::Right, 2),
        (TaxiRotation::Right, 2),
        (TaxiRotation::Right, 2),
    )));

    assert_eq!(12, part_1(vec!(
        (TaxiRotation::Right, 5),
        (TaxiRotation::Left, 5),
        (TaxiRotation::Right, 5),
        (TaxiRotation::Right, 3),
    )));
}

#[test]
fn part_1_is_correct() {
    assert_eq!(332, part_1(load_instructions()));
}
