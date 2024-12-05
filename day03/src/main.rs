use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let pairs: Vec<(u32, u32)> = pattern
    .captures_iter(&contents)
    .map(|caps| {
        let x = caps.get(1).unwrap().as_str().to_string().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().to_string().parse().unwrap();
        (x, y)
    })
    .collect();

    let mut result: u32 = 0;

    for pair in &pairs {
        result += pair.0 * pair.1;
    }
    
    let pattern2 = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut op: u8 = 1; // start with "do"
    let mut result2: u32 = 0;

    for cap in pattern2.captures_iter(&contents) {
        let content = cap.get(0).unwrap().as_str().to_string();

        if content == "do()" {
            op = 1;
        } else if content == "don't()" {
            op = 0;
        } else if op == 1 {
            let x: u32 = cap.get(1).unwrap().as_str().to_string().parse().unwrap();
            let y: u32 = cap.get(2).unwrap().as_str().to_string().parse().unwrap();
            result2 += x * y;
        }

    }

    println!("{}", result);
    println!("{}", result2);
}
