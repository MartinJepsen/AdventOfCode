use std::collections::HashSet;
use std::fs::{canonicalize, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const INPUT_FILE: &str = "day_01/input.txt";

// Part 1
fn get_lists() -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let path = canonicalize(PathBuf::from(INPUT_FILE))?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut list_left = Vec::<i32>::new();
    let mut list_right = Vec::<i32>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (left, right) = line.split_at(line.find("   ").unwrap());
        list_left.push(left.parse().unwrap());
        list_right.push(right.trim().parse().unwrap());
    }

    list_left.sort();
    list_right.sort();

    Ok((list_left, list_right))
}

fn get_difference(list_left: &Vec<i32>, list_right: &Vec<i32>) -> u32 {
    list_left
        .iter()
        .zip(list_right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>()
}

// Part 2
fn get_similarity_score(list_left: Vec<i32>, list_right: Vec<i32>) -> i32 {
    list_left
        // Collect the unique elements in the left list
        .iter()
        .collect::<HashSet<&i32>>()
        .into_iter()
        // For each unique number, multiply the number by the number of times it appears in the right list
        .map(|l| l * list_right.iter().filter(|r| *r == l).count() as i32)
        .sum::<i32>()
}

fn main() -> Result<(), std::io::Error> {
    // Part 1
    let (list_left, list_right) = get_lists()?;
    let summed_difference = get_difference(&list_left, &list_right);
    println!(
        "The sum of the absolute differences between the two lists is: {}",
        summed_difference
    );
    // Part 2
    let similarity_score = get_similarity_score(list_left, list_right);
    println!(
        "The similarity score between the two lists is: {}",
        similarity_score
    );
    Ok(())
}
