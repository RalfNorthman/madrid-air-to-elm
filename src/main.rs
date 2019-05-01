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
    BEN: Option<f64>,
    CH4: Option<f64>,
    CO: Option<f64>,
    EBE: Option<f64>,
    NMHC: Option<f64>,
    NO: Option<f64>,
    NO_2: Option<f64>,
    NOX: Option<f64>,
    O_3: Option<f64>,
    PM10: Option<f64>,
    PM25: Option<f64>,
    SO_2: Option<f64>,
    TCH: Option<f64>,
    TOL: Option<f64>,
    station: u32,
}

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
