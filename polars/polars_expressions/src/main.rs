use color_eyre::{Result};
use polars::prelude::*;

fn main() -> Result<()> {
    let s = Series::new("s", [1,2,3]);
    let t: Series = [1,2,3].iter().collect();
    println!("{}",t);
    Ok(())
}