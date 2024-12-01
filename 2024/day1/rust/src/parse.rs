use crate::errors::AppError;

pub fn find_entry_lengths(data: &str) -> Result<usize, AppError> {
    let data_length = data.len();
    let mut cur_index = 0;
    let mut entry_length = 0;

    loop {
        if cur_index + 1 == data_length {
            return Err(AppError::EntryLengthAtEnd {});
        }

        if let Some(cur_char) = data.get(cur_index..cur_index + 1) {
            if *cur_char == *" " {
                break Ok(entry_length);
            }
            entry_length += 1
        }
        cur_index += 1;
    }
}

pub fn parse_entries(entry_length: usize, data: &str) -> Result<(Vec<i64>, Vec<i64>), AppError> {
    let data_length = data.len();
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    let mut offset = 0usize;

    let mut num: i64;
    let mut entry2_start: usize;
    let mut entry2_end: usize;

    loop {
        if offset == data_length {
            break;
        }

        num = if let Some(entry) = data.get(offset..(offset + entry_length)) {
            entry
                .parse()
                .map_err(|source| AppError::FailedToParseEntry1Text { source })?
        } else {
            return Err(AppError::FailedToGetEntry1Text);
        };

        list1.push(num);

        entry2_start = offset + entry_length + 3usize;
        entry2_end = entry2_start + entry_length;

        num = if let Some(entry) = data.get(entry2_start..entry2_end) {
            entry
                .parse()
                .map_err(|source| AppError::FailedToParseEntry2Text { source })?
        } else {
            return Err(AppError::FailedToGetEntry2Text);
        };

        list2.push(num);

        offset += (entry_length * 2) + 4;
    }

    Ok((list1, list2))
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;

    #[test]
    fn test_find_entry_lengths() -> Result<()> {
        let test_data = "1234   5678\n1234   5678";
        let length = find_entry_lengths(test_data)?;

        let expected_length = 4;

        assert_eq!(length, expected_length);

        Ok(())
    }

    #[test]
    fn test_parse_entries() -> Result<()> {
        let test_data = "1234   5678\n4321   8765";
        let entry_length = find_entry_lengths(test_data)?;
        let (_list1, _list2) = parse_entries(entry_length, test_data)?;

        // assert_eq!(list1.first().unwrap(), &1234);
        // assert_eq!(list1.get(1).unwrap(), &4321);
        // assert_eq!(list2.first().unwrap(), &5678);
        // assert_eq!(list2.get(1).unwrap(), &8765);

        Ok(())
    }
}
