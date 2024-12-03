use std::env;
use std::fs;

use std::collections::LinkedList;

static INVALID_DIFF_SIZE: i32 = 3;

fn try_safe(line: &Vec<i32>, threshold: i32) -> Option<Vec<i32>> {
    let unsafe_indices: Vec<usize> = find_unsafe_indices(line);

    if threshold == 0 {
        if unsafe_indices.is_empty() {
            return Some(line.clone());
        } else {
            return None;
        }
    } else if unsafe_indices.is_empty() {
        return Some(line.clone());
    }

    // test every unsafe index
    for &index in unsafe_indices.iter() {
        let mut safe_line: Vec<i32> = line.clone();
        safe_line.remove(index);

        // did removing this unsafe index make the line safe?
        if find_unsafe_indices(&safe_line).is_empty() {
            return Some(safe_line);
        }
    }
    None
}

fn find_unsafe_indices(line: &[i32]) -> Vec<usize> {
    let mut unsafe_indices: Vec<usize> = vec![];
    let mut directions: LinkedList<i8> = LinkedList::new();

    for i in 1..line.len() {
        match line[i].cmp(&line[i-1]) {
            std::cmp::Ordering::Greater => {
                if line[i] - line[i-1] > INVALID_DIFF_SIZE {
                    unsafe_indices.push(i);
                }
                
                if directions.contains(&-1i8) {
                    unsafe_indices.push(i-1);
                    unsafe_indices.push(i);
                }

                directions.push_back(1);
            },
            std::cmp::Ordering::Less => { 
                if line[i-1] - line[i] > INVALID_DIFF_SIZE {
                    unsafe_indices.push(i);
                }

                if directions.contains(&1i8) {
                    unsafe_indices.push(i-1);
                    unsafe_indices.push(i);
                }

                directions.push_back(-1);
            },
            std::cmp::Ordering::Equal => unsafe_indices.push(i)
        }
    }

    unsafe_indices
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect::<Vec<&str>>();
    let num_lines: i32 = lines.len() as i32;

    // first 

    let mut num_safe: i32 = 0;
    let mut num_safe_threshold: i32 = 0;
    let threshold: i32 = 1;

    for line in lines {
        let split: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        if try_safe(&split, 0).is_some() {
            num_safe += 1;
        }

        if try_safe(&split, threshold).is_some() {
            num_safe_threshold += 1;
        }
    }

    println!("Num safe with threshold 0: {num_safe} out of {num_lines}");
    println!("Num safe with threshold {}: {} out of {}", threshold, num_safe_threshold, num_lines);
}
