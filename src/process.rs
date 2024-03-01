use std::collections::HashMap;

use crate::Record;

pub fn process_records(records: Vec<Record>) -> Vec<String> {
    let mut results = Vec::new();
    let mut cookies = HashMap::new();
    let mut max_count = 1;
    for r in records {
        match cookies.get_mut(&r.cookie) {
            None => {
                let _ = cookies.insert(r.cookie, 1);
            }
            Some(c) => {
                let count = *c + 1;
                if count > max_count {
                    max_count = count;
                }
                let _ = cookies.insert(r.cookie, count);
            }
        }
    }

    for cookie in cookies.keys() {
        let count = *cookies.get(cookie).unwrap();
        if count == max_count {
            results.push(cookie.clone());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use super::*;

    #[test]
    fn test_filter_by_date() {
        let data = vec!(
            Record::from("AtY0laUfhglK3lC7".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default())),
            Record::from("AtY0laUfhglK3lC7".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default())),
            Record::from("5UAVanZf6UtGyKVS".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default()))
        );

        let result = process_records(data);
        println!("{:?}", result);

        assert_eq!(result.len(), 1);
    }
}