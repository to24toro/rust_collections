fn main() {
    let v1 = vec![1,2,3];

    // for v in v1.iter() {
    //     println!("Got: {}", v);
    // }
    // let mut v1_iter = v1.iter();

    // assert_eq!(v1_iter.next(), Some(&1));
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);
    
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2,3,4]);
}

