use std::cell::Cell;

pub struct Immutable {
    a: Cell<i32>,
    b: i32,
}
fn main() {
    let x = Immutable{ a:Cell::new(0), b:0};
    x.a.set(x.a.get() + 1);
    // x.b += 1;
    println!("{:?}",x.a);

}
