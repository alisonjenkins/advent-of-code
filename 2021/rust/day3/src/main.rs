use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::io::Error;

use bitvec::prelude::*;

#[derive(Debug)]
struct Power {
    gamma: i64,
    epsilon: i64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_power_consumption(report_path: &Path) -> Result<Power, Error> {
    let power_state = Power {
        gamma: 0i64,
        epsilon: 0i64,
    };

    let mut report_data = vec![];

    if let Ok(lines) = read_lines(report_path) {
        for line in lines {
            let line = line.unwrap();
            report_data.push(parse_line(&line).unwrap());
        }
    }
    dbg!(report_data);
    Ok(power_state)
}

fn get_gamma_rate(report_data: &Vec<BitVec>) -> Result<u32, Error> {
    let num_ones = 0;
    let num_zeroes = 0;

    for row in report_data.iter() {
        for column in row.iter() {
            match column {
                true => {
                    num_ones += 1;
                },
                false => {
                    num_zeroes += 1;
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Result<BitVec, Error> {
    let mut output: BitVec = bitvec![];

    for character in line.chars() {
        match character {
            '1' => {
                output.push(true);
            },
            '0' => {
                output.push(false);
            },
            _ => {},
        }
    }

    Ok(output)
}

fn main() {
    let report_path = Path::new("./report.txt");
    let power_state = get_power_consumption(report_path);

    dbg!(power_state.unwrap());
}
