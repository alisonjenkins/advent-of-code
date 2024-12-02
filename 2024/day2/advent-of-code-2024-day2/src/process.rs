#[derive(Debug, PartialEq)]
pub enum Safety {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
pub enum DataTrend {
    Increasing,
    Decreasing,
}

pub fn are_readings_safe(readings: &Vec<i32>) -> Safety {
    let mut prev_reading: Option<i32> = None;
    let mut trend: Option<DataTrend> = None;

    for reading in readings {
        let last_reading = if let Some(prev_reading) = prev_reading {
            prev_reading
        } else {
            prev_reading = Some(*reading);
            continue;
        };

        if last_reading < *reading && trend.is_none() {
            trend = Some(DataTrend::Increasing);
        } else if last_reading > *reading && trend.is_none() {
            trend = Some(DataTrend::Decreasing);
        } else if last_reading > *reading && trend == Some(DataTrend::Increasing)
            || last_reading < *reading && trend == Some(DataTrend::Decreasing)
        {
            return Safety::Unsafe;
        }

        if *reading == last_reading {
            return Safety::Unsafe;
        }

        // if difference is too great
        let diff = if last_reading < *reading {
            *reading - last_reading
        } else {
            last_reading - *reading
        };

        if diff > 3 {
            return Safety::Unsafe;
        }

        prev_reading = Some(*reading);
    }

    Safety::Safe
}

pub fn problem_dampener_safe_records(readings: &Vec<i32>) -> Safety {
    let mut prev_reading: Option<i32> = None;
    let mut trend: Option<DataTrend> = None;
    let mut problem_previously_dampened = false;

    for reading in readings {
        // get the last reading, if not set skip to the next reading after setting previous
        let last_reading = if let Some(prev_reading) = prev_reading {
            prev_reading
        } else {
            prev_reading = Some(*reading);
            continue;
        };

        // Check the trend. If unset set it, if set check if the current data matches the trend
        if last_reading < *reading && trend.is_none() {
            trend = Some(DataTrend::Increasing);
        } else if last_reading > *reading && trend.is_none() {
            trend = Some(DataTrend::Decreasing);
        } else if (last_reading > *reading && trend == Some(DataTrend::Increasing)
            || last_reading < *reading && trend == Some(DataTrend::Decreasing))
            && !problem_dampened(&mut problem_previously_dampened)
        {
            return Safety::Unsafe;
        }

        // Check if the data has changed... if not unsafe
        if *reading == last_reading && !problem_dampened(&mut problem_previously_dampened) {
            return Safety::Unsafe;
        }

        // Check if difference is too great
        let diff = if last_reading < *reading {
            *reading - last_reading
        } else {
            last_reading - *reading
        };

        if diff > 3 && problem_dampened(&mut problem_previously_dampened) {
            return Safety::Unsafe;
        }

        prev_reading = Some(*reading);
    }

    Safety::Safe
}

fn problem_dampened(has_been_used: &mut bool) -> bool {
    if *has_been_used {
        return false;
    }

    *has_been_used = true;
    true
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::parse::parse_data;
    use color_eyre::eyre::Result;

    #[test]
    fn test_are_readings_safe() -> Result<()> {
        let records = parse_data(&PathBuf::from("../day-2-1-test-data.txt"))?;

        assert_eq!(are_readings_safe(&records[0]), Safety::Safe);
        assert_eq!(are_readings_safe(&records[1]), Safety::Unsafe);
        assert_eq!(are_readings_safe(&records[2]), Safety::Unsafe);
        assert_eq!(are_readings_safe(&records[3]), Safety::Unsafe);
        assert_eq!(are_readings_safe(&records[4]), Safety::Unsafe);
        assert_eq!(are_readings_safe(&records[5]), Safety::Safe);

        Ok(())
    }

    #[test]
    fn test_problem_dampened() -> Result<()> {
        let mut problem_previously_dampened = false;
        let output = problem_dampened(&mut problem_previously_dampened);

        // previously dampened now true
        assert!(problem_previously_dampened);

        // problem was dampened
        assert!(output);

        let output = problem_dampened(&mut problem_previously_dampened);

        // problem not damepend if previously dampened
        assert!(!output);

        Ok(())
    }

    #[test]
    fn test_problem_dampener_safe_records() -> Result<()> {
        let records = parse_data(&PathBuf::from("../day-2-2-test-data.txt"))?;

        assert_eq!(problem_dampener_safe_records(&records[0]), Safety::Safe);
        assert_eq!(problem_dampener_safe_records(&records[1]), Safety::Unsafe);
        assert_eq!(problem_dampener_safe_records(&records[2]), Safety::Unsafe);
        assert_eq!(problem_dampener_safe_records(&records[3]), Safety::Safe);
        assert_eq!(problem_dampener_safe_records(&records[4]), Safety::Safe);
        assert_eq!(problem_dampener_safe_records(&records[5]), Safety::Safe);

        Ok(())
    }
}
