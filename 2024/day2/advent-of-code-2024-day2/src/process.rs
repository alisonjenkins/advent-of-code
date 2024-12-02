pub enum Safety {
    Safe,
    Unsafe,
}

pub enum DataTrend {
    Increasing,
    Decreasing,
}

pub fn are_readings_safe(readings: Vec<i32>) -> Safety {
    let mut prev_reading: Option<i32> = None;

    for reading in readings {
        let last_reading = if let Some(prev_reading) = prev_reading {
            prev_reading
        } else {
            prev_reading = Some(reading);
            continue;
        };
        // if last_reading < cur
    }

    Safety::Safe
}
