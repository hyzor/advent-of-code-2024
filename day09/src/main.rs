mod file_block;

use file_block::FileBlock;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn has_empty_blocks_in_between(arr: &[FileBlock]) -> bool {
    let mut non_negative_encountered = false;
    for (i, x) in arr.iter().enumerate() {
        if x.id == -1 && non_negative_encountered && i < arr.len() - 1 {
            for y in arr[(i + 1)..].iter() {
                if y.id >= 0 {
                    return true;
                }
            }
        }
        if x.id >= 0 {
            non_negative_encountered = true;
        }
    }
    false
}

fn defragment(disk_blocks: &mut [FileBlock], blocks_moved: &mut u32) {
    for i in 0..disk_blocks.len() {
        if disk_blocks[i].id == -1 {
            for j in (0..disk_blocks.len()).rev() {
                if disk_blocks[j].id != -1 {
                    // swap positions
                    disk_blocks.swap(i, j);
                    *blocks_moved += 1;
                    break;

                }
            }
        }

        if !has_empty_blocks_in_between(disk_blocks) {
            break;
        }
    }
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    println!("In file {file_path}");

    let file: File = File::open(file_path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    let mut disk_map: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");

        for char in line.chars() {
            let digit = char.to_digit(10).unwrap() as u8;
            disk_map.push(digit);
        }
    }

    let mut disk_blocks: Vec<FileBlock> = Vec::new();
    let mut file_num = 0;

    for i in 0..disk_map.len() {
        let digit = disk_map[i];

        // even means file
        if (i % 2) == 0 {
            disk_blocks.extend(vec![FileBlock { id: file_num, value: char::from_digit(digit as u32, 10).unwrap() }; digit as usize]);
            file_num += 1;
        }
        // odd means free space
        else {
            disk_blocks.extend(vec![FileBlock { id: -1, value: '.' }; digit as usize]);
        }
    }

    let disk_size = disk_blocks.len();
    let mut blocks_moved = 0;

    defragment(&mut disk_blocks, &mut blocks_moved);

    println!("File count: {}", file_num);
    println!("Disk size: {}", disk_size);
    println!("Blocks moved: {}", blocks_moved);

    // calculate new checksum
    let mut checksum: u64 = 0;

    for i in 0..disk_blocks.len() {
        if disk_blocks[i].id != -1 {
            checksum += i as u64 * disk_blocks[i].id as u64;
        }
    }

    println!("Checksum: {}", checksum);

    Ok(())
}
