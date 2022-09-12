pub trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str {return "Geometry"}
}

struct Rectangle { width: u32, height: u32 }

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
    fn name(&self) -> &str { return "Rectangle" }
}

struct Triangle {bottom: u32, height: u32 }

impl Geometry for Triangle {
    fn area(&self) -> f64 {
        self.bottom as f64 * self.height as f64 * 0.5
    }
    fn name(&self) -> &str {return "Triangle"}
}


use std::fmt::Display;

struct Pair<T> { x: T, y:T }

impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self {x, y}
    }
}
impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is ys = {}", self.y);
        }
    }
}

fn main() {
    let a = Rectangle { width: 10, height: 20};
    let b = Triangle { bottom: 20, height: 10};

    println!("{} area={}",a.name(), a.area());
    println!("{} area={}",b.name(), b.area());
}
