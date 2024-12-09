use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn find_occurences(city_map: &[Vec<char>], char: char) -> Vec<(i32, i32)> {
    let mut occurences: Vec<(i32, i32)> = Vec::new();

    for (i, _) in city_map.iter().enumerate() {
        for j in 0..city_map[i].len() {
            if city_map[i][j] == char {
                occurences.push((i as i32, j as i32));
            }
        }
    }

    occurences
}

fn out_of_bounds((x1, y1): (i32, i32), width: i32, height: i32) -> bool {
    x1 < 0 || x1 >= width || y1 < 0 || y1 >= height
}

fn offset_coordinates(coord1: (i32, i32), coord2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    // Calculate the vector between the two coordinates
    let vector = (coord2.0 - coord1.0, coord2.1 - coord1.1);
    
    // Scale the vector by a factor of -1 and add it to the first coordinate
    let coord3 = (coord1.0 - vector.0, coord1.1 - vector.1);
    
    // Scale the vector by a factor of 1 and add it to the second coordinate
    let coord4 = (coord2.0 + vector.0, coord2.1 + vector.1);
    
    (coord3, coord4)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut city_map: Vec<Vec<char>> = Vec::new();
    let mut antenna_map: Vec<Vec<char>>;
    
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let chars: Vec<char> = line.chars().collect();
        city_map.push(chars);
    }

    antenna_map = city_map.clone();

    let width: i32 = city_map[0].len() as i32;
    let height: i32 = city_map.len() as i32;

    // hashmap of character occurences with coordinates
    let mut occurences_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..city_map.len() {
        for j in 0..city_map[i].len() {
            // is this a char of interest?
            if city_map[i][j] != '.' && city_map[i][j] != '#' && !occurences_map.contains_key(&city_map[i][j]) {
                let occurences: Vec<(i32, i32)> = find_occurences(&city_map, city_map[i][j]);
                occurences_map.insert(city_map[i][j], occurences);
            }
        }
    }

    // Print the 2D array
    for line in &city_map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    let mut antenna_coords: Vec<(i32, i32)> = Vec::new();

    for occurences in occurences_map.values() {
        for i in 0..occurences.len() - 1 {
            for j in i + 1..occurences.len() {
                let offset_coords: ((i32, i32), (i32, i32)) = offset_coordinates(occurences[i], occurences[j]);

                if !out_of_bounds(offset_coords.0, width, height) && !antenna_coords.contains(&offset_coords.0) {
                    antenna_coords.push(offset_coords.0);
                    antenna_map[offset_coords.0.0 as usize][offset_coords.0.1 as usize] = '#';
                }

                if !out_of_bounds(offset_coords.1, width, height) && !antenna_coords.contains(&offset_coords.1) {
                    antenna_coords.push(offset_coords.1);
                    antenna_map[offset_coords.1.0 as usize][offset_coords.1.1 as usize] = '#';
                }
            }
        }
    }

    for line in &antenna_map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    println!("Number of antennas: {}", antenna_coords.len());
    println!("Width: {}, Height: {}", width, height);

    Ok(())
}
