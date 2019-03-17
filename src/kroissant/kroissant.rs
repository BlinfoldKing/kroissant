use rand::{ thread_rng, Rng };

pub struct Classifier {
    train_data: Vec<Vec<f64>>,
    test_data: Vec<Vec<f64>>,
    k_value: i64
}

impl Classifier {
    pub fn new(train_data: Vec<Vec<f64>>, test_data: Vec<Vec<f64>>) -> Classifier {
        Classifier {
            train_data: train_data,
            test_data: test_data,
            k_value: 0
        }
    }

    fn calculate_distance(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
        let mut result: f64 = 0.0;
        return if v1.len() != v2.len() { 0_f64 }
        else {
            for i in 0..v1.len() {
                result += (v2[i] - v1[i]).powf(2.0);
            }
            result.sqrt()
        }
    }

    pub fn train(&mut self, min_k: i64, max_k: i64, validation_iteration: i64) {
        let mut accuracy = 0.0;
        for i in min_k..(max_k + 1) {
            let current_accuracy = self.validate(validation_iteration, i);
            println!("curr_k_value = {}, current_accuracy = {}", i, current_accuracy);
            accuracy = if accuracy < current_accuracy {
                self.k_value = i;
                current_accuracy
            } else {
                accuracy
            };
        }

        println!("choosen k_value = {}", self.k_value);
    }

    pub fn validate(&mut self, iteration: i64, k_value: i64) -> f64 {
        let raw_data = self.train_data.clone();

        let mut data: Vec<Vec<f64>> = Vec::new();
        thread_rng().shuffle(&mut data);

        let mut class: Vec<f64> = Vec::new();

        raw_data
            .into_iter()
            .for_each(|d| {
                let (last, value) = d.split_last().unwrap();
                data.push(value.to_vec());
                class.push(*last);
            });

        let validation_size = data.len() / iteration as usize;
        let mut valid_count = 0;
        let mut unvalid_count = 0;
        for i in 0..iteration {
            let begin = i as usize * validation_size;
            let end = begin + validation_size;
            let validation_data: Vec<Vec<f64>> = data[begin..end].to_vec();
            // println!("{} {}", begin, end);
            for j in 0..validation_data.len() {
                let mut distances: Vec<(i64, f64)> = Vec::new();
                data
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(x, _)| { *x < begin || *x > end })
                    .for_each(|(c, d)| {
                        distances
                            .push((
                                c as i64,
                                Classifier::calculate_distance(&validation_data[j as usize], &d)
                            ));
                    });
                distances.sort_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap());

                // naive counter
                let mut current_class = 0;
                let mut zero_counter = 0;
                let mut one_counter = 0;
                for distance in distances[0..k_value as usize].iter() {
                    let (index, _) = distance;
                    if class[*index as usize] as i64 == 1 {
                        one_counter += 1;
                    }
                    if class[*index as usize] as i64 == 0 {
                        zero_counter += 1;
                    }

                    current_class = if one_counter > zero_counter { 1 } else { 0 };
                }

                valid_count += if class[(begin + j)] as i64 == current_class { 1 } else { 0 };
                unvalid_count += if class[(begin + j)] as i64 != current_class { 1 } else { 0 };
                // for debugging
                // println!("k_value = {}, valid_count = {}, unvalid_count = {}", k_value, valid_count, unvalid_count);
            }
        }
        valid_count as f64 / (valid_count + unvalid_count) as f64
    }

    pub fn generate_result(&mut self) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();

        let raw_data = self.train_data.clone();
        let mut train_data: Vec<Vec<f64>> = Vec::new();
        let mut class: Vec<f64> = Vec::new();
        raw_data
            .into_iter()
            .for_each(|d| {
                let (last, value) = d.split_last().unwrap();
                train_data.push(value.to_vec());
                class.push(*last);
            });

        let test_data = self.test_data.clone();
        for data in test_data.into_iter() {
            let mut distances: Vec<(i64, f64)> = Vec::new();
            train_data
                .clone()
                .into_iter()
                .enumerate()
                .for_each(|(c, d)| {
                    distances
                        .push((
                            c as i64,
                            Classifier::calculate_distance(&data, &d)
                        ));
                });
            distances.sort_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap());

            // naive counter
            let mut current_class = 0;
            let mut zero_counter = 0;
            let mut one_counter = 0;
            for distance in distances[0..self.k_value as usize].iter() {
                let (index, _) = distance;
                if class[*index as usize] as i64 == 1 {
                    one_counter += 1;
                }
                if class[*index as usize] as i64 == 0 {
                    zero_counter += 1;
                }
            }
            current_class = if one_counter > zero_counter { 1 } else { 0 };
            result.push(current_class as f64);
        }
        result
    }
}

