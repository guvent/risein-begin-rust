use std::collections::HashMap;
use std::collections::HashSet;


fn vec_iter_sum() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_sum: i32 = numbers
        .iter()
        .filter(|n| n % 2 == 0)
        .sum();

    println!("Sum of even numbers: {}", even_sum);
}

fn hashmap_collect() {
    let numbers = vec![1, 2, 3, 4, 5];
    let number_squares: HashMap<_, _> = numbers
        .iter()
        .map(|n| (n, n * n))
        .collect();

    println!("{:?}", number_squares);
}

fn hashset_collect() {
    let numbers = vec![1, 2, 3, 4, 5];
    let unique_squares: HashSet<_> = numbers
        .iter()
        .map(|n| n * n)
        .collect();

    println!("{:?}", unique_squares);
}

fn vec_collect() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled_numbers: Vec<_> = numbers.iter().map(|n| n * 2).collect();

    println!("{:?}", doubled_numbers);
}

////// Chaining Iterator Methods //////

// Chaining multiple iterator methods
fn chain_multi_iter() {
    let words = vec!["apple", "banana", "cherry", "date", "fig"];
    let result: Vec<_> = words
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(i, w)| format!("{}: {}", i + 1, w.to_uppercase()))
        .collect();

    println!("{:?}", result);
}

// Chaining map and filter methods
fn chain_map_filter() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled_evens: Vec<_> = numbers
        .iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * 2)
        .collect();

    println!("{:?}", doubled_evens);
}

////// Chaining Iterator Methods //////



// fold like is reduce in javascript...
fn vec_fold() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

    println!("The sum of the numbers is: {}", sum); // Output: The sum of the numbers is: 15
}
fn vec_filter() {
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    println!("{:?}", even_numbers); // Output: [2, 4]
}

fn vec_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled); // Output: [2, 4, 6, 8, 10]
}
fn hash_for() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);

    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }
}

fn vec_for() {
    let numbers = vec![1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }

    for i in 0..5 {
        println!("{}", i);
    }
}
