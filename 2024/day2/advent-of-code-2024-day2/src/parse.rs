use crate::error::AppError;

pub fn parse_data(path: &std::path::PathBuf) -> Result<Vec<Vec<i32>>, AppError> {
    let data = std::fs::read_to_string(path).map_err(|source| AppError::FailedToReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    let mut records: Vec<Vec<i32>> = Vec::new();

    for record in data.lines() {
        let mut record_data = Vec::new();
        for num in record.split(" ") {
            record_data.push(
                num.parse()
                    .map_err(|source| AppError::FailedToParseNumber {
                        num: num.to_string(),
                        source,
                    })?,
            );
        }
        records.push(record_data);
    }

    Ok(records)
}
