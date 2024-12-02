use std::path::PathBuf;

use advent_of_code_2024_day2::{
    parse::parse_data,
    process::{self, Safety},
};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input_file_path = PathBuf::from("../day2-data.txt");
    let data = parse_data(&input_file_path)?;

    let mut safe_records = 0;
    let mut problem_dampener_safe_records = 0;
    for record in data {
        if process::are_readings_safe(&record) == Safety::Safe {
            safe_records += 1;
        }
        if process::problem_dampener_safe_records(&record) == Safety::Safe {
            problem_dampener_safe_records += 1;
        }
    }

    println!("Safe records: {safe_records}");
    println!("Problem Dampener safe records: {problem_dampener_safe_records}");

    Ok(())
}
