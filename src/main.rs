extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

use chrono::NaiveDateTime;
use std::error::Error;
use std::io;
use std::process;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Record {
    date: NaiveDateTime,
    ben: Option<f64>,
    ch4: Option<f64>,
    co: Option<f64>,
    ebe: Option<f64>,
    nmhc: Option<f64>,
    no: Option<f64>,
    no_2: Option<f64>,
    nox: Option<f64>,
    o_3: Option<f64>,
    pm10: Option<f64>,
    pm25: Option<f64>,
    so_2: Option<f64>,
    tch: Option<f64>,
    tol: Option<f64>,
    station: u32,
}

impl Record {
    fn to_elm(&self) -> String {
        format!(
            ", Record {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            to_posix(self.date),
            to_maybe(self.ben),
            to_maybe(self.ch4),
            to_maybe(self.co),
            to_maybe(self.ebe),
            to_maybe(self.nmhc),
            to_maybe(self.no),
            to_maybe(self.no_2),
            to_maybe(self.nox),
            to_maybe(self.o_3),
            to_maybe(self.pm10),
            to_maybe(self.pm25),
            to_maybe(self.so_2),
            to_maybe(self.tch),
            to_maybe(self.tol),
            self.station
        )
    }
}

fn to_posix(ndt: NaiveDateTime) -> String {
    let millis: i64 = ndt.timestamp_millis();
    format!("(Posix {})", millis)
}

fn to_maybe(opt: Option<f64>) -> String {
    match opt {
        None => String::from("Nothing"),
        Some(f) => format!("(Just {})", f),
    }
}

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{}", record.to_elm());
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
