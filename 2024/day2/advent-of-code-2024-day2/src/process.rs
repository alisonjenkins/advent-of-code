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
}
