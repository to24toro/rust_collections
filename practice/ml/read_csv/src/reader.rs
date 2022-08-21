use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::vec::Vec; 

#[derive(Debug)]
pub struct HousingDataset{
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64, 
    black: f64,
    lstat: f64, 
    medv: f64,
}

impl HousingDataset{
    pub fn new(v: Vec<&str>) -> HousingDataset {
        let unwrapped_text: Vec<f64> = v.iter().map(|r| r.parse().unwrap()).collect(); 
        HousingDataset{crim: unwrapped_text[0], 
                        zn: unwrapped_text[1], 
                        indus: unwrapped_text[2],
                        chas: unwrapped_text[3],
                        nox: unwrapped_text[4],
                        rm: unwrapped_text[5],
                        age: unwrapped_text[6],
                        dis: unwrapped_text[7],
                        rad: unwrapped_text[8],
                        tax: unwrapped_text[9],
                        ptratio: unwrapped_text[10],
                        black: unwrapped_text[11],
                        lstat: unwrapped_text[12],
                        medv: unwrapped_text[13]} 
    }

    pub fn train_features(&self) -> Vec<f64> {
        vec![self.crim, 
            self.zn,
            self.indus, 
            self.chas, 
            self.nox, 
            self.rm, 
            self.age, 
            self.dis, 
            self.rad, 
            self.tax, 
            self.ptratio, 
            self.black, 
            self.lstat]
    }

    pub fn train_target(&self) -> f64 {
        self.medv
    }
}

fn read_single_line(s: String) -> HousingDataset { 
    let raw_vector: Vec<&str> = s.split_whitespace().collect(); 
    let housing_vector: HousingDataset = HousingDataset::new(raw_vector); 
    housing_vector
}
pub fn read_housing_csv(filename: impl AsRef<Path>) -> Vec<HousingDataset> {
    let file = File::open(filename).expect("Please, give an input file. Cannot find file");
    let reader = BufReader::new(file);
    reader.lines().enumerate()
            .map(|(numb, line)| line.expect(&format!("Impossible to read line number {}", numb)))
            .map(|row| read_single_line(row))
            .collect()
}