use std::fs::File;
use std::io::Error;
use std::io::{self, BufRead};
use std::path::Path;

use bitvec::prelude::*;

#[derive(Debug)]
struct Power {
    gamma: i64,
    epsilon: i64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
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

fn get_gamma_rate(report_data: &Vec<BitVec>) -> Result<u32, std::io::Error> {
    let num_ones = 0;
    let num_zeroes = 0;
    let gamma_rate: u32 = 0;
    let is_first_reading = true;

    // peek at first row to get the length of rows
    let record_length = report_data[0].as_bitslice().len();

    // for each column in the numbers
    // if the number is a 1 add to num_ones
    // if the number is a 0 add to num_zeroes
    //
    // if whatever is the most common digit is column n's value of the gamma rate where n is the
    // current number of the column.
    //
    // the epsilon rate is calculated the same way but instead the least common bit for each volumn
    // is used.
    //
    // power rate is the decimal version of gamma and epsilon rates multiplied together
    // (gamma*epsilon).

    for row in report_data.iter() {
        let row_data = row.as_bitslice();

        // identify the length of the rows in the data
        if is_first_reading {
            record_length = row_data.len();
        }

        for 

        match bitvec::ptr::read(column.into()) {
            true => {
                num_ones += 1;
            }
            false => {
                num_zeroes += 1;
            }
        }
    }

    Ok(gamma_rate)
}

fn parse_line(line: &str) -> Result<BitVec, Error> {
    let mut output: BitVec = bitvec![];

    for character in line.chars() {
        match character {
            '1' => {
                output.push(true);
            }
            '0' => {
                output.push(false);
            }
            _ => {}
        }
    }

    Ok(output)
}

fn main() {
    let report_path = Path::new("./report.txt");
    let power_state = get_power_consumption(report_path);

    dbg!(power_state.unwrap());
}
