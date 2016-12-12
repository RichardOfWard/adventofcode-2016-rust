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

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
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
                TaxiOrientation::North => (self.position.0, self.position.1 + 1),
                TaxiOrientation::South => (self.position.0, self.position.1 - 1),
                TaxiOrientation::East => (self.position.0 + 1, self.position.1),
                TaxiOrientation::West => (self.position.0 - 1, self.position.1),
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
    position.0.abs() + position.1.abs()
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

fn find_last_taxi_distance(instructions: Vec<Instruction>) -> i64 {
    taxi_abs(generate_taxi_history(instructions).last().unwrap().position)
}

fn find_first_duplicate_taxi_distance(instructions: Vec<Instruction>) -> Option<i64> {
    let history = generate_taxi_history(instructions)
        .into_iter()
        .map(|taxi| taxi.position)
        .collect::<Vec<Position>>();

    history
        .iter()
        .filter(|position| {
            history.iter().filter(|p| position == p).count() > 1
        })
        .map(|p| taxi_abs(*p))
        .next()
}

pub fn part_1() -> i64 {
    find_last_taxi_distance(load_instructions())
}

pub fn part_2() -> i64 {
    find_first_duplicate_taxi_distance(load_instructions()).unwrap()
}

#[test]
fn taxicab_abs_solves_the_part_1_examples() {
    assert_eq!(5, find_last_taxi_distance(vec!(
        (TaxiRotation::Right, 2),
        (TaxiRotation::Left, 3),
    )));

    assert_eq!(2, find_last_taxi_distance(vec!(
        (TaxiRotation::Right, 2),
        (TaxiRotation::Right, 2),
        (TaxiRotation::Right, 2),
    )));

    assert_eq!(12, find_last_taxi_distance(vec!(
        (TaxiRotation::Right, 5),
        (TaxiRotation::Left, 5),
        (TaxiRotation::Right, 5),
        (TaxiRotation::Right, 3),
    )));
}

#[test]
fn part_1_is_correct() {
    assert_eq!(332, part_1());
}

#[test]
fn taxicab_abs_solves_the_part_2_examples() {
    assert_eq!(4, find_first_duplicate_taxi_distance(vec!(
        (TaxiRotation::Right, 8),
        (TaxiRotation::Right, 4),
        (TaxiRotation::Right, 4),
        (TaxiRotation::Right, 8),
    )).unwrap());

    assert_eq!(0, find_first_duplicate_taxi_distance(vec!(
        (TaxiRotation::Right, 1),
        (TaxiRotation::Right, 1),
        (TaxiRotation::Right, 1),
        (TaxiRotation::Right, 1),
    )).unwrap());

    assert_eq!(2, find_first_duplicate_taxi_distance(vec!(
        (TaxiRotation::Right, 5),
        (TaxiRotation::Left, 5),
        (TaxiRotation::Left, 3),
        (TaxiRotation::Left, 7),
    )).unwrap());
}

#[test]
fn part_2_is_correct() {
    assert_eq!(166, part_2());
}
