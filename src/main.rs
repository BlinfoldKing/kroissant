extern crate csv;

use std::error::Error;
use std::fs;

fn readCSV(from: &str) -> Result<Vec<Vec<String>>, Box<Error>> {
    let content = fs::read_to_string(from).unwrap();
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut data: Vec<Vec<String>> = Vec::new();
    for result in reader.records() {
        let result = result?;
        let mut newRow:Vec<String> = Vec::new();
        for col in &result { 
           newRow.push(String::from(col));
        }
        data.push(newRow);
    }
    Ok(data)
}

fn main() {
    let trainData = readCSV("src/data/TrainData.csv").unwrap();
    let testData = readCSV("src/data/TestData.csv").unwrap();
   println!("{:?}", trainData[0]);
   println!("{:?}", testData[0]);
}
