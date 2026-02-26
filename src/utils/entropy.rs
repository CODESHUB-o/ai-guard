pub fn calculate_entropy(input: &str) -> f64 {
    let mut frequency = std::collections::HashMap::new();
    let len = input.len() as f64;

    if len == 0.0 {
        return 0.0;
    }

    for c in input.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }

    let mut entropy = 0.0;

    for count in frequency.values() {
        let p = *count as f64 / len;
        entropy -= p * p.log2();
    }

    entropy
}