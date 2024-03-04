use std::collections::HashMap;

use crate::Cookie;

// Process records to find most seen cookie
pub fn process_cookies(cookies: Vec<Cookie>) -> Vec<String> {
    let mut results = Vec::new();
    let mut cookie_map = HashMap::new();
    let mut max_count = 1;
    for c in cookies {
        match cookie_map.get_mut(&c.cookie) {
            None => {
                let _ = cookie_map.insert(c.cookie, 1);
            }
            Some(cnt) => {
                let count = *cnt + 1;
                if count > max_count {
                    max_count = count;
                }
                let _ = cookie_map.insert(c.cookie, count);
            }
        }
    }

    for cookie in cookie_map.keys() {
        let count = *cookie_map.get(cookie).unwrap();
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
    fn test_process_records() {
        let data = vec!(
            Cookie::from("AtY0laUfhglK3lC7".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default())),
            Cookie::from("AtY0laUfhglK3lC7".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default())),
            Cookie::from("5UAVanZf6UtGyKVS".to_string(), NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), NaiveTime::default()))
        );

        let result = process_cookies(data);
        println!("{:?}", result);

        assert_eq!(result.len(), 1);
    }
}