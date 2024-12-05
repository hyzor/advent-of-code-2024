use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_to_2d_map(file_path: &str) -> Vec<Vec<char>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Failed to read line").chars().collect())
        .collect()
}

fn is_x_mas_pair(input: char, input2: char) -> bool {
    return (input == 'M' || input == 'S') && (input2 == 'M' || input2 == 'S') && input != input2;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    let input: Vec<Vec<char>> = read_file_to_2d_map(file_path);

    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut total_matches = 0;
    let mut total_x_mas_matches = 0;

    for i in 0..num_rows {
        for j in 0..num_cols {
            if input[i][j] == 'X' {
                let mut directions = vec![];
                // test up
                if i > 2 && input[i - 1][j] == 'M' && input[i - 2][j] == 'A' && input[i - 3][j] == 'S' {
                    directions.push("UP");
                }

                // test up left
                if i > 2 && j > 2 && input[i - 1][j - 1] == 'M' && input[i - 2][j - 2] == 'A' && input[i - 3][j - 3] == 'S' {
                    directions.push("UP LEFT");
                }

                // test left
                if j > 2 && input[i][j - 1] == 'M' && input[i][j - 2] == 'A' && input[i][j - 3] == 'S' {
                    directions.push("LEFT");
                }

                // test up right
                if i > 2 && j < num_cols - 3 && input[i - 1][j + 1] == 'M' && input[i - 2][j + 2] == 'A' && input[i - 3][j + 3] == 'S' {
                    directions.push("UP RIGHT");
                }

                // test right
                if j < num_cols - 3 && input[i][j + 1] == 'M' && input[i][j + 2] == 'A' && input[i][j + 3] == 'S' {
                    directions.push("RIGHT");
                }

                // test down left
                if i < num_rows - 3 && j > 2 && input[i + 1][j - 1] == 'M' && input[i + 2][j - 2] == 'A' && input[i + 3][j - 3] == 'S' {
                    directions.push("DOWN LEFT");
                }

                // test down right
                if i < num_rows - 3 && j < num_cols - 3 && input[i + 1][j + 1] == 'M' && input[i + 2][j + 2] == 'A' && input[i + 3][j + 3] == 'S' {
                    directions.push("DOWN RIGHT");
                }

                // test down
                if i < num_rows - 3 && input[i + 1][j] == 'M' && input[i + 2][j] == 'A' && input[i + 3][j] == 'S' {
                    directions.push("DOWN");
                }

                if directions.len() > 0 {
                    total_matches += directions.len();
                }
            } else if input[i][j] == 'A' {
                // first check if within bounds
                if (i > 0 && i < num_rows - 1) && (j > 0 && j < num_cols - 1) {
                    let mut neighbors = vec![];
                    neighbors.extend([input[i - 1][j - 1], input[i + 1][j + 1], input[i - 1][j + 1], input[i + 1][j - 1]]);

                    // if neighbors are M and S and idx 0,1 and 2,3 does not equal each other, then it's a match
                    if is_x_mas_pair(neighbors[0], neighbors[1]) &&
                        is_x_mas_pair(neighbors[2], neighbors[3]) {
                        total_x_mas_matches += 1;
                    }
                }
            }
        }
    }

    println!();
    println!("Total matches: {}", total_matches);
    println!("Total X-MAS matches: {}", total_x_mas_matches);
    println!("Rows: {}, Cols: {}", num_rows, num_cols);
}
