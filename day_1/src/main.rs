use std::fs;

fn main() {
    let input_data = match fs::read_to_string("input.txt") {
        Ok(data) => data,
        Err(_) => panic!("Something went wrong reading the file"),
    };

    let mut numbers: Vec<String> = Vec::new();
    let mut buff = String::new();

    for char in input_data.chars() {
        if char.is_ascii_digit() {
            buff.push(char);
        } else if !buff.is_empty() {
            numbers.push(buff);
            buff = String::new();
        }
    }

    let mut left: Vec<String> = Vec::new();
    let mut right: Vec<String> = Vec::new();

    let mut go_left = true;

    for number in numbers {
        if go_left {
            left.push(number);
            go_left = false;
        } else {
            right.push(number);
            go_left = true;
        }
    }

    left.sort();
    right.sort();

    let mut distances: Vec<i32> = Vec::new();

    for i in 0..left.len() {
        let a = left[i].parse::<i32>().unwrap();
        let b = right[i].parse::<i32>().unwrap();

        let distance = floor(a - b);
        distances.push(distance);
    }

    println!("sum of distances: {}", distances.iter().sum::<i32>());
}

fn floor(n: i32) -> i32 {
    if n > 0 {
        n
    } else {
        n * -1
    }
}
