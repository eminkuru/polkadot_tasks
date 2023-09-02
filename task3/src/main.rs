fn main() {
    let some_vector = vec![
        "naber".to_string(),
        "iyi".to_string(),
        "senden?".to_string(),
    ];

    let data = FilterCondition {
        value: "naber".to_string(),
    };

    let filtered_vector = custom_filter(some_vector.clone(), &data);

    println!("{:?}", some_vector);
    println!("{:?}", filtered_vector);
}

struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.value == item
    }
}

fn custom_filter<T: PartialEq>(vector: Vec<T>, cond: &FilterCondition<T>) -> Vec<T> {
    let new_vector: Vec<T> = vector
        .into_iter()
        .filter(|item| cond.is_match(item))
        .collect();

    return new_vector;
}
