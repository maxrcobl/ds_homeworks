fn main() {
    let total: i32 = scores.iter().sum();
    let mut scores = vec![85, 92, 78, 96];
    scores.push(88);
    let average = total as f64 / scores.len() as f64;

    if average >= 90.0 {
    println!("Good work! Average: {:.1}", average);
    } else if average >= 80.0 {
    println!("Excellent! Average: {:.1}", average);

    } else {
    println!("Keep trying! Average: {:.1}", average);

}}


