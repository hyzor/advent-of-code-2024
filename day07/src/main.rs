use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// This function takes a slice of u32s, a target number, and a boolean indicating
/// whether or not to concatenate adjacent numbers. It returns a boolean indicating
/// whether or not the target number can be reached by adding, multiplying, and/or
/// concatenating the numbers in the slice.
fn calculate_combinations(nums: &[u32], target: u64, concat: bool) -> bool {
    let mut current_results: Vec<u64> = vec![nums[0] as u64];
    for &num in &nums[1..] {
        let mut new_results: Vec<u64> = Vec::new();
        for &result in &current_results {
            new_results.push(result + num as u64);
            new_results.push(result * num as u64);
            if concat {
                let concatenated_num: u64 = format!("{}{}", result, num).parse::<u64>().unwrap();
                new_results.push(concatenated_num);
            }
        }
        current_results = new_results;
    }
    current_results.contains(&target)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let file: File = File::open(file_path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);
    
    let mut equations: Vec<(u64, Vec<u32>)> = Vec::new();
    let mut failed_equations: Vec<(u64, Vec<u32>)> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");
        let split: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let first: u64 = split[0].parse().unwrap();
        let second: Vec<u32> = split[1].trim_start().split(" ").map(|s| s.parse().unwrap()).collect();

        equations.push((first, second.clone()));
    }

    println!("Number of equations: {}", equations.len());

    let mut sum: u64 = 0;

    for (i, nums) in equations.iter() {
        let is_found: bool = calculate_combinations(nums, *i, false);

        if is_found {
            sum += i;
        } else {
            failed_equations.push((*i, nums.clone()));
        }
    }

    println!("Number of failed equations: {}", failed_equations.len());

    let mut concat_sum: u64 = 0;

    // Now try the failed equations, try to concatenate each adjacent digits
    // to find a solution
    for (i, nums) in failed_equations.iter() {
        let is_found: bool = calculate_combinations(nums, *i, true);

        if is_found {
            concat_sum += i;
        }
    }

    println!("Sum: {sum}");
    println!("Concat sum: {concat_sum}");
    println!("Sum + Concat sum: {}", sum + concat_sum);

    Ok(())
}
