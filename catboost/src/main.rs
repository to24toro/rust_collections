use catboost;

fn sigmoid(x: f64) -> f64 {
    1. / (1. +(-x).exp())
}

fn answer(makes_over_50k_a_year: bool) -> &'static str {
    if makes_over_50k_a_year {
        "make over 50k a year"
    } else {
        "doesn't make over 50k a year"
    }
}
fn main() {
    let model_path = "adult.cbm";
    let model = catboost::Model::load(model_path).unwrap();

    println!("Adult dataset model metainformation\n");

    println!("tree count: {}", model.get_tree_count());

    println!("prediction dimension: {}", model.get_dimensions_count());

    println!("numeric feature count: {}", model.get_float_features_count());

    println!("categoric feature count: {}", model.get_cat_features_count());

    println!();

    let person_a_numeric_features = vec![25., 226_802., 7., 0., 0., 40.];
    let person_a_categoric_features = vec![
        String::from("Private"),
        String::from("11th"),
        String::from("Never-married"),
        String::from("Machine-op-inspct"),
        String::from("Own-child"),
        String::from("Black"),
        String::from("Male"),
        String::from("United-States"),
    ];
    let person_a_prediction = model
    .calc_model_prediction(
        vec![person_a_numeric_features],
        vec![person_a_categoric_features],
    )
    .unwrap();

    let person_a_makes_over_50k_probability = sigmoid(person_a_prediction[0]);
    println!(
        "Person A make over 50K a year with probability {}",
        person_a_makes_over_50k_probability
    );

    let classification_threshold = 0.5;
    let person_a_makes_over_50k = person_a_makes_over_50k_probability > classification_threshold;
    println!("Person A {}", answer(person_a_makes_over_50k));

    println!();
    let person_b_numeric_features = vec![40., 85019., 16., 0., 0., 45.];
    let person_b_categoric_features = vec![
        String::from("Private"),
        String::from("Doctorate"),
        String::from("Married-civ-spouce"),
        String::from("Prof-specialty"),
        String::from("Husband"),
        String::from("Asian-Pac-Islander"),
        String::from("Male"),
        String::from("nan"),
    ];
    let person_b_prediction = model
        .calc_model_prediction(
            vec![person_b_numeric_features.clone()],
            vec![person_b_categoric_features.clone()],
        )
        .unwrap();
    let person_b_makes_over_50k_probability = sigmoid(person_b_prediction[0]);
    let person_b_makes_over_50k = person_b_makes_over_50k_probability > classification_threshold;
    println!(
        "Person B make over 50K a year with probability {}",
        person_b_makes_over_50k_probability
    );
    println!("Person B {}", answer(person_b_makes_over_50k));

    let persons_ab_numberic_features = vec![person_a_numeric_features, person_b_numeric_features];
    let persons_ab_categoric_features = vec![person_a_categoric_features, person_b_categoric_features];
    let persons_ab_predictions = model
        .calc_model_prediction(persons_ab_numberic_features, persons_ab_categoric_features)
        .unwrap();
    let persons_ab_make_over_50k_probabilities =
        vec![sigmoid(persons_ab_predictions[0]), sigmoid(persons_ab_predictions[1])];
    let persons_ab_make_over_50k = vec![
        persons_ab_make_over_50k_probabilities[0] > classification_threshold,
        persons_ab_make_over_50k_probabilities[1] > classification_threshold,
    ];

    println!("Using batch interface");

    println!(
        "Person A make over 50K a year with probability {}",
        persons_ab_make_over_50k_probabilities[0]
    );
    println!("Person A {}", answer(persons_ab_make_over_50k[0]));
    println!(
        "Person B make over 50K a year with probability {}",
        persons_ab_make_over_50k_probabilities[1]
    );
    println!("Person B {}", answer(persons_ab_make_over_50k[1]));
}
