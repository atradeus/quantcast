use std::path::Path;

use crate::Record;

//
// Read and parse given CSV file
//
pub fn parse_file(file: &Path) -> Result<Vec<Record>, String> {
    // defaults to skip first record as header
    let mut reader = match csv::Reader::from_path(file) {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to read csv file {:?}: {}", file, e).to_string())
    };

    // Deserialize records and create list
    let mut records: Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        // unwrap result
        let record: Record = match record {
            Ok(r) => r,
            Err(e) => return Err(format!("Failed to parse record: {}", e)),
        };
        // .expect(&format!("Invalid record"));
        records.push(record);
    }

    if records.is_empty() {
        return Err("File is empty".to_string());
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_parse_file() {
        let mut tmpfile = match NamedTempFile::new() {
            Ok(f) => f,
            Err(_) => panic!("Failed to create temp file")
        };

        write!(tmpfile, r#"cookie,timestamp
AtY0laUfhglK3lC7,2018-12-09T14:19:00+00:00
SAZuXPGUrfbcn5UA,2018-12-09T10:13:00+00:00
5UAVanZf6UtGyKVS,2018-12-09T07:25:00+00:00
AtY0laUfhglK3lC7,2018-12-09T06:19:00+00:00
SAZuXPGUrfbcn5UA,2018-12-08T22:03:00+00:00
4sMM2LxV07bPJzwf,2018-12-08T21:30:00+00:00
fbcn5UAVanZf6UtG,2018-12-08T09:30:00+00:00
4sMM2LxV07bPJzwf,2018-12-07T23:30:00+00:00
"#).unwrap();

        let r = match parse_file(tmpfile.path()) {
            Ok(r) => r,
            Err(e) => panic!("Parse file failed {}", e)
        };
        assert_eq!(r.len(), 8);
    }

    #[test]
    fn test_bad_data() {
        let mut tmpfile = match NamedTempFile::new() {
            Ok(f) => f,
            Err(_) => panic!("Failed to create temp file")
        };

        write!(tmpfile, r#"cookie,timestamp
AtY0laUfhglK3lC7,foobar
SAZuXPGUrfbcn5UA,baddate
"#).unwrap();

        let r = parse_file(tmpfile.path());
        assert!(r.is_err())
    }

    #[test]
    fn test_missing_file() {
        let r = parse_file(Path::new("missing.csv"));
        assert!(r.is_err())
    }
}