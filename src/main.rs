use std::fs;
use apriori_algorithm::dataset::Dataset;


fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("small_items.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let dataset = Dataset::from_lines(&lines);
    println!("{:?}", dataset);
}
