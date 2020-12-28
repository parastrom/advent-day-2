use std::env;
use std::fs;

fn main() {
    let filename = "src\\input.txt";
    let content = fs::read_to_string(filename)
    .expect("Something went wrong with reading the file");

    let lines:Vec<&str> = content.split("\n").collect();
    let mut counter = 0;
    let length = lines.len();
    for x in 0..length {
        let indiv_line:Vec<&str> = lines[x].split(":").collect();
        let check = part_2(&indiv_line);
        if check {
            counter += 1;
        }
    }
    println!("{}",counter);
}


fn part_1(line:&Vec<&str>) -> bool {
    let mut check = false;
    let mut letter_count = 0;
    let pre_numbers:Vec<&str> = line[0].split(" ").collect();
    let numbers:(u32,u32) = {
            let y:Vec<&str> = pre_numbers[0].split("-").collect();
            (y[0].parse::<u32>().unwrap(), y[1].parse::<u32>().unwrap())
        };
    let letter = pre_numbers[1].chars().collect::<Vec<_>>()[0];
    let password:Vec<char> = line[1].trim().chars().collect();
    for i in 0..password.len() {
        if letter == password[i] { 
            letter_count += 1;
        }
    }
    if letter_count >= numbers.0 && letter_count <= numbers.1 {
        check = true;
    }
    check
}

fn part_2(line:&Vec<&str>) -> bool {
    let mut check = false;
    let pre_numbers:Vec<&str> = line[0].split(" ").collect();
    let numbers:(usize,usize) = {
            let y:Vec<&str> = pre_numbers[0].split("-").collect();
            (y[0].parse::<usize>().unwrap(), y[1].parse::<usize>().unwrap())
        };
    let letter = pre_numbers[1].chars().collect::<Vec<_>>()[0];
    let password:Vec<char> = line[1].trim().chars().collect();
    let first_pos_check = {
        password[numbers.0 - 1] == letter
    };
    let second_pos_check = {
        password[numbers.1 - 1] == letter
    };
    if first_pos_check ^ second_pos_check {
        check = true;
    }
    check
}