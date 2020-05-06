//tutorial-read-serde-04.rs
extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use math::round;
use std::error::Error;
use std::io;
use std::process;

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    size: i32,
    attribute: String,
}

fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<i32> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("original: {:#?}", record.size);
        let new_rec: f64 = (record.size as f64 / 3.0).into();
        println!("divided: {:#?}", new_rec);
        let new_rec = round::floor(new_rec, 0);
        println!("rounded: {:#?}", new_rec);
        let new_rec = new_rec as i32 - 2;
        println!("subtracted: {:#?}", new_rec);
        v.push(new_rec);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }
    let sum: i32 = v.iter().sum();
    println!("{:#?}", v);
    println!("{:#?}", sum);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
