extern crate core;

use std::fs::File;
use std::io::{Read};
use self::core::str::FromStr;

pub fn part_1() -> i64 {
    return taxi_abs(load_instructions());
}


fn load_instructions() -> Vec<Instruction> {
    read_file("inputs/1-1.txt")
        .trim()
        .split(", ")
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(str: &str) -> Instruction {
    (str[0..1].parse::<RelativeDirection>().unwrap(), str[1..].parse::<i64>().unwrap())
}

type Instruction = (RelativeDirection, i64);

fn taxi_abs(directions: Vec<Instruction>) -> i64 {
    let mut facing = AbsoluteDirection::North;

    let (north, east) = directions.into_iter()
        .map(|(relative_direction, distance)| {
            facing = facing.rotate(relative_direction);
            facing.as_vector(distance)
        })
        .fold((0, 0), |(n1, e1), (n2, e2)| (n1 + n2, e1 + e2));

    (north + east).abs()
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}

#[derive(Clone, Copy)]
enum RelativeDirection {
    Left,
    Right,
}

impl FromStr for RelativeDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(RelativeDirection::Left),
            "R" => Ok(RelativeDirection::Right),
            _ => Err("Unknown relative direction ".to_string() + s),
        }
    }
}

#[derive(Clone, Copy)]
enum AbsoluteDirection {
    North,
    East,
    South,
    West,
}

impl AbsoluteDirection {
    fn rotate(self, rel: RelativeDirection) -> AbsoluteDirection {
        match rel {
            RelativeDirection::Right => match self {
                AbsoluteDirection::North => AbsoluteDirection::East,
                AbsoluteDirection::East => AbsoluteDirection::South,
                AbsoluteDirection::South => AbsoluteDirection::West,
                AbsoluteDirection::West => AbsoluteDirection::North,
            },
            RelativeDirection::Left => match self {
                AbsoluteDirection::East => AbsoluteDirection::North,
                AbsoluteDirection::South => AbsoluteDirection::East,
                AbsoluteDirection::West => AbsoluteDirection::South,
                AbsoluteDirection::North => AbsoluteDirection::West,
            },
        }
    }

    fn as_vector(self, distance: i64) -> (i64, i64) {
        match self {
            AbsoluteDirection::North => (distance, 0),
            AbsoluteDirection::East => (0, distance),
            AbsoluteDirection::South => (-distance, 0),
            AbsoluteDirection::West => (0, -distance),
        }
    }
}

#[test]
fn taxicab_abs_solves_the_day_1_examples() {
    assert_eq!(5, taxi_abs(vec!(
        (RelativeDirection::Right, 2),
        (RelativeDirection::Left, 3),
    )));

    assert_eq!(2, taxi_abs(vec!(
        (RelativeDirection::Right, 2),
        (RelativeDirection::Right, 2),
        (RelativeDirection::Right, 2),
    )));

    assert_eq!(12, taxi_abs(vec!(
        (RelativeDirection::Right, 5),
        (RelativeDirection::Left, 5),
        (RelativeDirection::Right, 5),
        (RelativeDirection::Right, 3),
    )));
}

#[test]
fn part_1_is_correct() {
    assert_eq!(332, part_1());
}
