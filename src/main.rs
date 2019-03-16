extern crate csv;

mod kroissant;

use std::error::Error;
use std::fs;

fn read_csv(from: &str) -> Result<Vec<Vec<String>>, Box<Error>> {
    let content = fs::read_to_string(from).unwrap();
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut data: Vec<Vec<String>> = Vec::new();
    for result in reader.records() {
        let result = result?;
        let mut new_row:Vec<String> = Vec::new();
        for col in &result {
           new_row.push(String::from(col));
        }
        data.push(new_row);
    }
    Ok(data)
}

fn main() {
    let raw_train_data = read_csv("src/data/TrainData.csv").unwrap();
    let raw_test_data = read_csv("src/data/TestData.csv").unwrap();

    // convert data into floating point
    let train_data: Vec<Vec<f64>> =
        raw_train_data
        .into_iter()
        .map(|container| container
             .into_iter()
             .map(|value| value.parse().unwrap())
             .collect())
        .collect();
    let test_data: Vec<Vec<f64>> =
        raw_test_data
        .into_iter()
        .map(|container| container
             .into_iter()
             .map(|value| value.parse().unwrap())
             .collect())
        .collect();

}


