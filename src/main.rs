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

fn write_csv(data: Vec<f64>, to: &str) -> Result<(), Box<Error>> {
    let mut writer = csv::Writer::from_path(to)?;
    for d in data.into_iter() {
        writer.write_record(&[(d as u64).to_string()]);
    }
    Ok(())
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
    let unpruned_test_data: Vec<Vec<f64>> =
        raw_test_data
        .into_iter()
        .map(|container| container
             .into_iter()
             .map(|value| value.parse().unwrap_or(0.0))
             .collect())
        .collect();

    let test_data: Vec<Vec<f64>> =
        unpruned_test_data
        .into_iter()
        .map(|container| container[0..(container.len() - 1 as usize)].to_vec())
        .collect();
    let mut classifier =
        kroissant::Classifier::new(train_data, test_data);
    classifier.train(1, 100, 800);
    let class_result = classifier.generate_result();
    println!("generated class = {:?}", class_result);
    write_csv(class_result, "result.csv").unwrap();
}

