fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let nums = vec![432,35,542,23,5262,2,5];

    let result = largest(&nums);
    println!("The largest number is {}", result);

    let chars = vec!["SD", "#r", "gr", "as"];

    let result = largest(&chars);
    println!("The largest chars is {}", result);
}
