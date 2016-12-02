use std::fs::File;
use std::io::{Read};

fn main() {
    let mut file = File::open("inputs/1-1.txt").unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let directions = data.trim().split(", ").map(|str| {
        let direction: day1::Relative = str[0..1].parse().unwrap();
        let distance: i64 = str[1..].parse().unwrap();
        (direction, distance)
    }).collect();

    println!("{}", day1::taxi_abs(directions))
}

mod day1 {
    extern crate core;

    use self::core::str::FromStr;

    #[derive(Clone, Copy)]
    pub enum Relative {
        Left,
        Right,
    }

    impl FromStr for Relative {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "L" => Ok(Relative::Left),
                "R" => Ok(Relative::Right),
                _ => Err("Unknown relative direction ".to_string() + s),
            }
        }
    }

    #[derive(Clone, Copy)]
    enum Absolute {
        North,
        East,
        South,
        West,
    }

    impl Absolute {
        fn rotate(self, rel: Relative) -> Absolute {
            match rel {
                Relative::Right => match self {
                    Absolute::North => Absolute::East,
                    Absolute::East => Absolute::South,
                    Absolute::South => Absolute::West,
                    Absolute::West => Absolute::North,
                },
                Relative::Left => match self {
                    Absolute::East => Absolute::North,
                    Absolute::South => Absolute::East,
                    Absolute::West => Absolute::South,
                    Absolute::North => Absolute::West,
                },
            }
        }

        fn vector(self, distance: i64) -> (i64, i64) {
            match self {
                Absolute::North => (distance, 0),
                Absolute::East => (0, distance),
                Absolute::South => (-distance, 0),
                Absolute::West => (0, -distance),
            }
        }
    }


    pub fn taxi_abs(directions: Vec<(Relative, i64)>) -> i64 {
        let mut facing = Absolute::North;

        let (north, east) = directions
            .into_iter()
            .map(|(relative_direction, distance)| {
                facing = facing.rotate(relative_direction);
                facing.vector(distance)
            })
            .fold((0, 0), |(n1, e1), (n2, e2)| (n1 + n2, e1 + e2));

        (north + east).abs()
    }


    #[test]
    fn day1_taxi_abs() {
        assert_eq!(5, taxi_abs(vec!(
            (Relative::Right, 2),
            (Relative::Left, 3),
        )));

        assert_eq!(2, taxi_abs(vec!(
            (Relative::Right, 2),
            (Relative::Right, 2),
            (Relative::Right, 2),
        )));

        assert_eq!(12, taxi_abs(vec!(
            (Relative::Right, 5),
            (Relative::Left, 5),
            (Relative::Right, 5),
            (Relative::Right, 3),
        )));
    }
}
