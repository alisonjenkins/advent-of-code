use advent_of_code_2024_day_1::{
    errors::AppError,
    find_distance, find_simularity_score,
    parse::{find_entry_lengths, parse_entries},
};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input_path = std::path::PathBuf::from("../day1.txt");

    // read the file
    let data = std::fs::read_to_string(&input_path).map_err(|source| AppError::ReadData {
        path: input_path,
        source,
    })?;

    // find entry lengths
    let entry_length = find_entry_lengths(&data)?;

    // parse entries
    let (mut list1, mut list2) = parse_entries(entry_length, &data)?;

    // sort lists
    list1.sort();
    list2.sort();

    // find distances
    let total_entries = list1.len();
    let mut total_distance = 0i64;
    let mut num1;
    let mut num2;
    for entry in 0..total_entries {
        num1 = list1.get(entry).unwrap();
        num2 = list2.get(entry).unwrap();
        total_distance += find_distance(*num1, *num2);
    }

    println!("Total distance: {total_distance}");

    // find similarity score
    let simularity = find_simularity_score(&list1, &list2);
    println!("Simularity score: {simularity}");

    Ok(())
}
