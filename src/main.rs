extern crate core;

use std::path::Path;

use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use clap::{command, Parser};
use serde::Deserialize;

mod parse;
mod filter;
mod process;

//
#[derive(Parser, Debug)]

#[command(name = "Quantcast")]
#[command(version = "1.0")]
#[command(about = "Quantcast App", long_about = None)]
pub struct Cli {
    /// Date in format YYYY-MM-DD
    #[arg(short, long, value_name = "date")]
    date: NaiveDate,
    /// Input file
    #[arg(short, long, value_name = "file")]
    file: String,
}

// Cookie, id and timestamp
#[derive(Deserialize, Debug)]
pub struct Cookie {
    // Cookie identifier
    cookie: String,
    // Timestamp of when cookie was seen
    timestamp: DateTime<Utc>,
}

// For easier testing
impl Cookie {
    pub fn from(cookie: String, timestamp: NaiveDateTime) -> Cookie {
        // For this exercise, time and timezone are not relevant
        let date_time: DateTime<Utc> = Utc.from_local_datetime(&timestamp).unwrap();
        Cookie {
            cookie,
            timestamp: date_time,
        }
    }
}

fn run_app(file: String, date: NaiveDate) -> Vec<String> {
    // Parse file
    let records = match parse::parse_file(Path::new(&file)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Filter cookies
    let cookies = filter::filter_by_date(date, records);
    if cookies.is_empty() {
        eprintln!("Error: No records for date {}", date);
        std::process::exit(1);
    }

    process::process_cookies(cookies)
}


fn main() {
    let cli: Cli = Cli::parse();

    let results = run_app(cli.file, cli.date);
    for cookie in results {
        println!("{}", cookie);
    }
}
