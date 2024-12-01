use std::env;
use std::fs;

// function that pushes an i32 to a sorted vector
fn push_sorted(v: &mut Vec<i32>, i: i32) {
    let mut idx = 0;
    while idx < v.len() && v[idx] < i {
        idx += 1;
    }
    v.insert(idx, i);
}

// function that compares two i32 and returns the difference
fn compare(a: i32, b: i32) -> i32 {
    println!("{a} {b}");
    if a > b {
        a - b
    } else {
        b - a
    }
}

// function that counts how many times a number appears in a vector
fn count(v: &Vec<i32>, i: i32) -> i32 {
    let mut count = 0;
    for j in v {
        if *j == i {
            count += 1;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect::<Vec<&str>>();

    // two vectors of ints
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    // contents has two lists separated by three spaces, split these
    for line in lines {
        // split line
        let split = line.split("   ").collect::<Vec<&str>>();

        push_sorted(&mut v1, split[0].parse().unwrap());
        push_sorted(&mut v2, split[1].parse().unwrap());
    }

    let mut total_distance: i32 = 0;

    let mut similarity_score: i32 = 0;

    // assumes v1.len() == v2.len()
    for i in 0..v1.len() {
        total_distance += compare(v1[i], v2[i]);
        similarity_score += v1[i] * count(&v2, v1[i]);
    }

    println!("Total distance: {total_distance}");
    println!("Similarity score: {similarity_score}");
}
