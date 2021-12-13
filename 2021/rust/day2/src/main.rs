use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::io::Error;
use std::str::FromStr;
use std::num::ParseIntError;

enum Direction {
    Down,
    Forward,
    Up,
}

struct Command {
    direction: Direction,
    count: i64,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_vec = s.split(" ").collect::<Vec<&str>>();
        let count: i64 = command_vec[1].parse::<i64>().unwrap();

        let direction = match command_vec[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            &_ => Direction::Up,
        };

        Ok(Command {
            direction: direction,
            count: count,
        })
    }
}

struct Location {
    horizontal: i64,
    depth: i64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_location(commands_path: &Path) -> Result<Location, Error> {
    let mut location = Location {
        horizontal: 0i64,
        depth: 0i64,
    };

    if let Ok(lines) = read_lines(commands_path) {
        for line in lines {
            let command = line.unwrap().parse::<Command>().unwrap();
            match command.direction {
                Direction::Down => {
                    location.depth += command.count;
                },
                Direction::Up => {
                    location.depth -= command.count;
                },
                Direction::Forward => {
                    location.horizontal += command.count;
                },
            }
        }
    }
    Ok(location)
}

fn main() {
    let commands_path = Path::new("./commands.txt");
    let location = get_location(commands_path).unwrap();
    let output = location.horizontal * location.depth;

    println!("{}", output);
}
