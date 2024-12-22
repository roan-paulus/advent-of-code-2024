use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type RuleMap = HashMap<u8, Vec<u8>>;
const SIZE: usize = 16384;

fn main() {
    let mut f = File::open("input.txt").expect("There should be a file there");

    let mut buf: [u8; SIZE] = [0; SIZE];
    let bytes_read = f.read(&mut buf).unwrap();
    println!("Read {bytes_read} bytes");

    sum_correct_updates(buf);
}

fn sum_correct_updates(s: [u8; SIZE]) {
    let (rules, updates) = split_rules_updates(&s);
    let rules = parse_rules(rules);

    let mut incorrect_update_sum: u32 = 0;

    let mut correct_updates = vec![];
    for update in updates.split(|ch| *ch == '\n') {
        match parse_update(update) {
            Some(update) if is_correct_update(&rules, &update) => correct_updates.push(update),
            Some(mut update) => {
                order_incorrect_update(&rules, &mut update);
                if let Some(middle) = take_middle(update) {
                    incorrect_update_sum += middle as u32;
                }
            }
            _ => (),
        }
    }

    println!(
        "Incorrect corrected middle values are {}",
        incorrect_update_sum
    );

    assert!(!correct_updates.is_empty());

    let mut sum: u32 = 0;
    for update in correct_updates {
        if let Some(num) = take_middle(update) {
            sum += num as u32;
        } else {
            panic!("Shit");
        }
    }
    println!(
        "The sum of the middle numbers in the correct updates is: {}",
        sum
    );
}

fn order_incorrect_update(rules: &RuleMap, update: &mut [u8]) {
    let mut has_swapped = true;

    while has_swapped {
        has_swapped = false;

        for i in 0..update.len() - 1 {
            let number = update[i];
            let next_n = update[i + 1];

            if !is_correct(rules, number, next_n) {
                has_swapped = true;
                update[i] = next_n;
                update[i + 1] = number;
            }
        }
    }
}

fn parse_update(update: &[char]) -> Option<Vec<u8>> {
    if update.is_empty() {
        return None;
    }
    let res = update
        .split(|ch| *ch == ',')
        .map_while(convert_two_digit_number_to_u8)
        .collect::<Vec<u8>>();
    Some(res)
}

fn is_correct_update(rules: &RuleMap, numbers: &[u8]) -> bool {
    for i in 0..numbers.len() {
        let a = numbers[i];

        let next = numbers.get(i + 1);
        if next.is_none() {
            break;
        }

        let b = next.expect("Exists because of check earlier");
        if !is_correct(rules, a, *b) {
            return false;
        }
    }
    true
}

fn is_correct(rules: &RuleMap, a: u8, b: u8) -> bool {
    let rules = match rules.get(&b) {
        Some(rules) => rules,
        // There is no rule about this so it must be correct.
        None => return true,
    };

    for number in rules {
        if a == *number {
            return false;
        }
    }
    true
}

fn take_middle(chars: Vec<u8>) -> Option<u8> {
    let len = chars.len();

    if len == 0 {
        None
    } else {
        Some(chars[len / 2])
    }
}

fn split_rules_updates(input: &[u8]) -> (Vec<char>, Vec<char>) {
    let middle = input
        .windows(2)
        .position(|window| window == [b'\n', b'\n'])
        .expect("There should be two newlines seperating the rules from the updates");

    let rules = to_vec_chars(&input[..middle]);
    let updates = to_vec_chars(&input[middle..]);

    (rules, updates)
}

fn to_vec_chars(slice: &[u8]) -> Vec<char> {
    slice
        .iter()
        .filter(|byte| **byte != b'\0')
        .map(|byte| *byte as char)
        .collect()
}

fn parse_rules(rules: Vec<char>) -> RuleMap {
    let mut parsed_rules: RuleMap = HashMap::new();

    let rules = rules.split(|ch| *ch == '\n');
    for rule in rules {
        let mut rule = rule
            .split(|ch| *ch == '|')
            .map_while(convert_two_digit_number_to_u8);

        let pt_1 = rule.next().unwrap();
        let pt_2 = rule.next().unwrap();

        assert!(rule.next().is_none());

        parsed_rules.entry(pt_1).or_insert_with(|| vec![pt_2]);
        parsed_rules.get_mut(&pt_1).unwrap().push(pt_2);
    }
    parsed_rules
}

fn convert_two_digit_number_to_u8(chars: &[char]) -> Option<u8> {
    if chars.len() != 2 {
        return None;
    }
    let mut num: u8 = 0;
    let mut chars = chars.iter();
    num += (chars.next().unwrap()).to_digit(10).unwrap() as u8 * 10;
    num += (chars.next().unwrap()).to_digit(10).unwrap() as u8;
    Some(num)
}
