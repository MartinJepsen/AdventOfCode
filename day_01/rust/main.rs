use std::fs::{File, canonicalize};
use std::io::{BufReader, BufRead};
use std::path::PathBuf;


const INPUT_FILE: &str = "day_01/input.txt";

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

fn main() -> Result<(), std::io::Error> {
    let (list_left, list_right) = get_lists()?;
    let summed_difference = list_left.iter().zip(list_right.iter()).map(|(l, r)| l.abs_diff(*r)).sum::<u32>();
    
    println!("The sum of the absolute differences between the two lists is: {}", summed_difference);
    Ok(())
}
