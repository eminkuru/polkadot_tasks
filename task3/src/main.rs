fn main() {
    // Create a vector of desired type of data. I used strings.
    let some_vector = vec![
        "naber".to_string(),
        "iyi".to_string(),
        "senden?".to_string(),
    ];

    // Create a FilterCondition with a value
    let data = FilterCondition {
        value: "naber".to_string(),
    };

    // Call custom_filter to filter the vector according to the FilterCondition
    let filtered_vector = custom_filter(some_vector.clone(), &data);

    // Print the original and filtered vectors
    println!("{:?}", some_vector);
    println!("{:?}", filtered_vector);
}

// Define a generic struct FilterCondition
struct FilterCondition<T> {
    value: T,
}

// Implement methods for FilterCondition
impl<T> FilterCondition<T>
where
    T: PartialEq + PartialOrd,
{
    // Define a method is_match that checks if an item matches the value
    fn is_match(&self, item: &T) -> bool {
        &self.value == item
    }
}

// Define a generic function custom_filter
fn custom_filter<T>(vector: Vec<T>, cond: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + PartialOrd,
{
    // Create a new vector by filtering elements based on the condition
    let new_vector: Vec<T> = vector
        .into_iter()
        .filter(|item| cond.is_match(item))
        .collect();

    return new_vector;
}
