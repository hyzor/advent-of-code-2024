use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn pair_exists(line: &[u16], pair: (u16, u16)) -> bool {
    line.contains(&pair.0) && line.contains(&pair.1)
}

fn validate_line(line: &[u16], rules: &[(u16, u16)]) -> bool {
    let filtered_rules: Vec<(u16, u16)> = rules.iter().filter(|rule| pair_exists(line, **rule)).cloned().collect();

    let mut valid: bool = true;

    for rule in filtered_rules {
        if line.iter().position(|&x| x == rule.0).unwrap() > line.iter().position(|&x| x == rule.1).unwrap() {
            valid = false;
            break;
        }
    }

    valid
}

// I totally wrote this function by myself and did not use AI :)
fn rearrange_elements(line: &[u16], rules: &[(u16, u16)]) -> Vec<u16> {
    let mut dependencies: HashMap<u16, Vec<u16>> = HashMap::new();
    let filtered_rules: Vec<(u16, u16)> = rules.iter().filter(|rule| pair_exists(line, **rule)).cloned().collect();

    for (first, second) in filtered_rules {
        dependencies.entry(second).or_default().push(first);
    }

    let mut result: Vec<u16> = Vec::new();
    let mut visited: HashSet<u16> = HashSet::new();

    fn visit(node: u16, dependencies: &HashMap<u16, Vec<u16>>, result: &mut Vec<u16>, visited: &mut HashSet<u16>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);

        if let Some(deps) = dependencies.get(&node) {
            for dep in deps {
                visit(*dep, dependencies, result, visited);
            }
        }

        result.push(node);
    }

    for &node in line {
        visit(node, &dependencies, &mut result, &mut visited);
    }

    result.reverse();
    result
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut rules: Vec<(u16, u16)> = Vec::new();
    let mut updates: Vec<Vec<u16>> = Vec::new();
    let mut reading_rules = true;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            reading_rules = false;
        } else if reading_rules {
            let numbers_array: Vec<u16> = line.split("|").map(|s| s.parse().unwrap()).collect();
            rules.push((numbers_array[0], numbers_array[1]));
        } else {
            let numbers_array: Vec<u16> = line.split(",").map(|s| s.parse().unwrap()).collect();
            updates.push(numbers_array);
        }
    }

    let mut valid_sum: u32 = 0;
    let mut fixed_lines_sum: u32 = 0;

    for line in updates {
        if validate_line(&line, &rules) {
            valid_sum += line[line.len() / 2] as u32;
        } else {
            let fixed_line = rearrange_elements(&line, &rules);
            fixed_lines_sum += fixed_line[fixed_line.len() / 2] as u32;
        }
    }

    println!("Valid sum: {valid_sum}");
    println!("Fixed sum: {fixed_lines_sum}");
    Ok(())
}
