use std::io::{self};
use yijia_ids706_mini_proj7::stats;

fn main() {
    println!("Enter a list of numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the input into a vector of f64
    let mut numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    if numbers.is_empty() {
        println!("No valid numbers provided.");
        return;
    }

    // Calculate the mean
    match stats::mean(&numbers) {
        Some(mean) => println!("Mean: {}", mean),
        None => println!("No data provided for mean."),
    }

    // Calculate the median
    match stats::median(&mut numbers) {
        Some(median) => println!("Median: {}", median),
        None => println!("No data provided for median."),
    }
}
