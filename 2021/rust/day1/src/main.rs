use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::io::Error;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_scan(scan_path: &Path) -> Result<Vec<u32>, Error> {
    let mut scan_numbers = Vec::new();

    if let Ok(lines) = read_lines(scan_path) {
        for line in lines {
            let num = line.unwrap().parse::<u32>().unwrap();
            scan_numbers.push(num);
        }
    }
    Ok(scan_numbers)
}

fn main() {
    let scan_path = Path::new("./sonar_scan.txt");
    let scan_numbers = read_scan(scan_path).unwrap();

    let mut increase_count = 0;
    let mut first_iteration = true;
    let mut prev_number: u32 = 0;
    for scan_number in scan_numbers {
        if first_iteration {
            prev_number = scan_number;
            first_iteration = false;
            continue;
        }

        if scan_number > prev_number {
            increase_count += 1;
        }

        prev_number = scan_number;
    }

    println!("{}",increase_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_scan() {
        let path = Path::new("./sonar_scan.txt");
        let scans = read_scan(path).unwrap();
        assert_eq!(scans[0], 199);
    }
}
