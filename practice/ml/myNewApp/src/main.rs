fn main() {
    let name = "Kento";
    let name_surname = add_surname(&name);
    println!("{:?}", name_surname);

    let nickname = add_nickname(&name);
    println!("{:?}", nickname);

    let myself = Person{
        name: "Kento".to_string(),
        surname: "Ueda".to_string(),
        age: 30,
    };

    println!("I was born in the {}", myself.compute_year_of_birth());

}

fn add_surname(s: &str) -> String {
    s.to_string() + " Ueda"
}

fn add_nickname(s: &str) -> String {
    s.to_string() + " Ue"
}

struct Person {
    name: String,
    surname: String,
    age: u32,
}

trait PersonSpec {
    fn compute_year_of_birth(&self) -> u32;
}

impl PersonSpec for Person {
    fn compute_year_of_birth(&self) -> u32 {
        2021 - self.age
    }
}

