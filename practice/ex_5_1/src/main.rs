struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username:String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}


fn main() {
    let mut user1 = build_user(String::from("another@example.com"), String::from("Tanaka Taro"));
    let user2 = User {
        email: String::from("other@example.com"),
        username: String::from("Suzuki Toru"),
        ..user1
    };
    println!("{}", user2.email);
    
}
