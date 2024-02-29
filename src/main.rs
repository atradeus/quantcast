#[allow(dead_code, unused_imports)]
//TODO REMOVE above allow
use std::fs::File;

use chrono::{DateTime, NaiveDate, Utc};
use clap::{command, Parser};
use serde::Deserialize;
use std::collections::HashMap;

//
#[derive(Parser, Debug)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Quantcast app", long_about = None)]
pub struct Cli {
    /// Date in format YYYY-MM-DD.
    #[arg(short, long, value_name = "date")]
    date: NaiveDate,

    /// Input file
    #[arg(short, long, value_name = "file")]
    file: String,
}

#[derive(Deserialize, Debug)]
pub struct Record {
    cookie: String,
    timestamp: DateTime<Utc>,
}

fn parse(file: String) -> Vec<Record> {
    let mut reader =  csv::Reader::from_path(file.clone())
        .expect(&format!("Failed to read csv file {}", file));

    let mut records: Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        let record: Record = record.expect(&format!("Invalid record {:?}", record));
        records.push(record);
    }

    records
}

fn filter(date: NaiveDate, records: Vec<Record>) -> Vec<Record> {
    let mut filtered: Vec<Record> = Vec::new();
    for r in records {
        if r.timestamp.date_naive() == date {
            filtered.push(r);
        }
    }

    filtered
}

fn process(records: Vec<Record>) {
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
            println!("{}", cookie);
        }
    }
}

fn main() {
    let cli: Cli = Cli::parse();
    println!("{:#?}", cli);

    let records = parse(cli.file);
    if records.is_empty() {
        panic!("Empty file")
    }

    let records = filter(cli.date, records);
    if records.is_empty() {
        panic!("No records with given date {}", cli.date)
    }
    println!("{:?}", records);

    process(records);
}
