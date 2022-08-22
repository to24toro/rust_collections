use polars::prelude::*;
use polars::prelude::{Result as PolarResult};
use polars_core::prelude::*;
use polars::frame::DataFrame;
use std::fs::File;
use std::path::{Path};

use polars::prelude::SerReader;

pub fn read_csv<P: AsRef<Path>>(path: P) -> PolarResult<DataFrame> {
    let file = File::open(path).expect("Cannot open file.");

    CsvReader::new(file).has_header(true).finish()
}

pub fn deal_with_shape<P: AsRef<Path>>(path: Path) -> () {
    let df = read_csv(&path).unwrap();

    let shape = df.shape();
    println!("{:#?}", shape);
    println!("{:#?}", df.schema());
    println!("{:#?}", df.dtypes());

    let width = df.width();
    println!("{}", width);
    let height = df.height();
    println!("{}", height);

}

pub fn deal_with_columns<P: AsRef<Path>>(path: P) -> () {
    let df = read_csv(&path).unwrap();

    let columns = df.get_columns();
    let columname = df.get_column_names();

    for (i, column) in columns.iter().enumerate() {
        println!("{}, {}", column, columname[i]);
    }
}

pub fn deal_with_stacks<P: AsRef<Path>>(path: P) -> () {
    println!("Read the same dataframe twice");
    let df = read_csv(&path).unwrap();
    let df2 = read_csv(&path).unwrap();
    println!("Vertical stac the two dataframes");
    let mut df3 = df.vstack(&df2).unwrap(); // mut --> so we can change this dataframe later
    println!("{}, {:#?}", df3.head(Some(5)), df3.shape());
    // get column 
    println!("Get a column");
    let  sepal_length = df3.column("sepal.length").unwrap();
    println!("{}", sepal_length);
    println!("{:#?}", sepal_length.len());

    // drop columns
    println!("Drop a column");
    let sepal_length = df3.drop_in_place("sepal.length").unwrap(); // inplace
    println!("{}", df3.head(Some(5)));
    println!("Insert a series in a dataframe as a new column");
    let _df4 = df3.insert_at_idx(0, sepal_length).unwrap();
    println!("{}", _df4.head(Some(5)));
}
fn numb_to_log(in_df: &mut DataFrame) -> PolarResult<Series>{
    // do with a series  unwrap to have Series, .f64().uwrap() to retrieve a chunked array
    let to_log10_column = in_df.drop_in_place("sepal.length").unwrap().rename("log10.sepal.length")
                                                             .f64().unwrap() // create chunked array
                                                             .cast::<Float64Type>() // here we have apply
                                                             .unwrap() // unwrap because we have Result<>
                                                             .apply(|s| s.log10());
    
    let series10 = to_log10_column.into_series(); // reconvert into a series
    
    // return the column
    println!("{}", series10);
    Ok(series10)
}

pub fn deal_with_apply<P: AsRef<Path>>(path: P) -> () {
    let mut df = read_csv(&path).unwrap();

    df.apply_at_idex(0, |s| s + 1);

    let log10_series = numb_to_log(&mut df);
    df.with_column(log10_series.unerap());
    df.apply_at_idx(0, |s| s.f64().unwrap().apply(|t| t.log10()));
}

fn main() {
    let ifile = "../iris.csv";
    // shape info 
    deal_with_shape(&ifile);
    // columns info 
    deal_with_columns(&ifile);
    // concatenate dataframe 
    deal_with_stacks(&ifile);
    // do math on this 
    deal_with_apply(&ifile);
    // train test split
    // nd array conversion, 2D only 
    let ifile = "../datasets/2d_array.csv";
    let df = read_csv(&ifile).unwrap();
    let ndarray = df.to_ndarray::<Float64Type>().unwrap();
    println!("{:?}", ndarray);
}
