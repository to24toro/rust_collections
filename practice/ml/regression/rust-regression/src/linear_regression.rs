use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::vec::Vec;


use rusty_machine;
use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::BaseMatrix;
use rusty_machine::linalg::Vector;
use rusty_machine::learning::lin_reg::LinRegressor;
use rusty_machine::learning::SupModel;

use rusty_machine::analysis::score::neg_mean_squared_error;

use read_csv::read_housing_csv;


pub fn run() -> f64 {
    let ifile = "../datasets/boston_house.csv";
    let mut input_data = read_housing_csv(&ifile);

    let test_chunk_size: f64 = input_data.len() as f64 * 0.3;
    let test_chunk_size = test_chunk_size.round() as usize;

    let (test, train) = input_data.split_at(test_chunk_size);

    let train_size = train.len();
    let test_size = test.len();

    let x_train: Vec<f64> = train.iter().flat_map(|row| row.train_features()).collect();
    let y_train: Vec<f64> = train.iter().map(|row| row.train_target()).collect();

    let x_test: Vec<f64> = test.iter().flat_map(|row| row.train_features()).collect();
    let y_test: Vec<f64> = test.iter().map(|row| row.train_target()).collect();


    let x_train_matrix = Matrix::new(train_size, 13, x_train);
    let y_train_vector = Vector::new(y_train);
    let x_test_matrix = Matrix::new(test_size, 13, x_test);

    let mut linearRegression = LinRegressor::default();

    linearRegression.train(&x_train_matrix, &y_train_vector);

    let preds = linearRegression.predict(&x_test_matrix).unwrap();

    let preds_matrix = Matrix::new(test_size, 1, preds);
    let y_preds_matrix = Matrix::new(test_size, 1, y_test);

    let mse = neg_mean_squared_error(&preds_matrix, &y_preds_matrix);

    println!("Final negMSE (the higher the better) {:?}", mse);

    mse
    
}
