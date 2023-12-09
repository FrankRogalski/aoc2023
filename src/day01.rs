use std::fs;
use std::error::Error;
use regex::Regex;
use std::collections::HashMap;

fn number_to_add(first: Option<char>, last: Option<char>) -> u32 {
    if let Some(first) = first {
        if let Some(last) = last {
            let mut num = String::with_capacity(2);
            num.push(first);
            num.push(last);
            let num: u32 = num.parse().unwrap();
            return num;
        } else {
            panic!("no numbers found");
        }
    }
    panic!("no numbers found");
}

fn set_first_and_last(chr: char, first: &mut Option<char>, last: &mut Option<char>) {
    if first.is_none() {
        *first = Some(chr);
    }
    *last = Some(chr);
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day01.txt")?;
    let mut sum = 0;
    for line in res.lines() {
        let mut first = Option::None;
        let mut last = Option::None;
        for chr in line.chars() {
            if chr.is_ascii_digit() {
                set_first_and_last(chr, &mut first, &mut last);
            }
        }
        sum += number_to_add(first, last);
    }
    println!("the sum of the numbers for part 1 is {sum}");
    Ok(())
}

fn to_char(string: &str, values: &HashMap<String, u32>) -> char {
    if string.parse::<u8>().is_ok() {
        string.chars().next().unwrap()
    } else {
        char::from_u32(values.get(string).expect(string) + 0x30).unwrap()
    }
}

fn overlaps(s1: &str, s2: &str) -> Vec<String> {
    let mut overlaps = Vec::new();
    for (i, (c1, c2)) in s1.chars().zip(s2.chars().rev()).enumerate() {
        if c1 == c2 {
            let s = s2[..s2.len() - i].to_string() + &s1[i + 1..];
            overlaps.push(s);
        } else {
            return overlaps;
        }
    }
    overlaps
}

fn generate_map_and_regex() -> (HashMap<String, u32>, Regex) {
    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut values = HashMap::new();
    let mut regex = String::new();
    for (i1, n1) in NUMBERS.iter().enumerate() {
        for (i2, n2) in NUMBERS.iter().enumerate() {
            if n1 != n2 {
                for overlap in overlaps(n1, n2) {
                    values.insert(overlap.clone(), i2 as u32 + 1);
                    regex.push_str(&(overlap + "|"));
                }
            }
        }
        regex.push_str(&(n1.to_string() + "|"));
        values.insert(n1.to_string(), i1 as u32 + 1);
    }
    regex.push_str(r"\d");
    dbg!(&regex);
    (values, Regex::new(&regex).unwrap())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day01.txt")?;
    let mut sum = 0;
    let (values, regex) = generate_map_and_regex();
    for line in res.lines() {
        let matches: Vec<_> = regex.find_iter(line).collect();
        let first = to_char(matches.first().unwrap().as_str(), &values);
        let last = to_char(matches.last().unwrap().as_str(), &values);
        sum += number_to_add(Some(first), Some(last));
    }
    println!("the sum of the numbers for part 2 is {sum}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
