use chrono::NaiveDate;

use crate::Record;

// Filter records by given date
// returns filtered list
pub fn filter_by_date(date: NaiveDate, records: Vec<Record>) -> Vec<Record> {
    let mut filtered: Vec<Record> = Vec::new();
    for r in records {
        if r.timestamp.date_naive() == date {
            filtered.push(r);
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, NaiveTime};

    use super::*;

    #[test]
    fn test_filter_by_date() {
        let data = vec!(
            Record::from("AtY0laUfhglK3lC7".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default())),
            Record::from("5UAVanZf6UtGyKVS".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 2).unwrap(), NaiveTime::default()))
        );


        let filtered = filter_by_date(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), data);

        assert_eq!(filtered.len(), 1);
    }
}