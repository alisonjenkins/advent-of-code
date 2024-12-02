use std::path::PathBuf;

use advent_of_code_2024_day2::parse::parse_data;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    // read / parse data
    let input_file_path = PathBuf::from("../day-2-1-test-data.txt");
    let data = parse_data(&input_file_path)?;
    println!("{data:?}");

    // process data

    // output data

    Ok(())
}
