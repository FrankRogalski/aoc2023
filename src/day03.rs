use std::fs;
use std::error::Error;

fn build_number(start_id: usize, res_chars: &Vec<char>) -> i32 {
    let mut id = start_id;
    while res_chars.get(id).unwrap().is_ascii_digit() {
        id -= 1;
    }
    id += 1;
    let mut num = String::new();
    loop {
        let chr = res_chars.get(id).unwrap();
        if chr.is_ascii_digit() {
            num.push(*chr);
        } else {
            break;
        }
        id += 1;
    }
    num.parse::<i32>().unwrap()
}

fn check_numbers(line_len: usize, line_num: isize, chr_num: usize, res_chars: &Vec<char>) -> Vec<i32> {
    let mut res = vec![];
    for y in -1 as isize..=1 {
        if line_num == 0 && y == -1 {
            continue;
        }
        let mut previos_num = false;
        let id = line_len
            * (line_num + y) as usize
            + chr_num;
        for x in -1 as isize..=1 {
            if chr_num == 0 && x == -1 {
                continue;
            }
            let id = (id as isize + x) as usize;
            if let Some(chr) = res_chars.get(id) {
                if chr.is_ascii_digit() {
                    previos_num = true;
                } else if previos_num {
                    previos_num = false;
                    res.push(build_number(id - 1, res_chars));
                }
            }
        }
        if previos_num {
            let id = line_len 
                * (line_num + y) as usize 
                + (chr_num + 1) as usize;
            res.push(build_number(id, res_chars));
        }
    }
    res
}

fn check_neighbors(line_len: usize, line_num: isize, chr_num: isize, res_chars: &Vec<char>) -> bool {
    for y in -1 as isize..=1 {
        for x in -1 as isize..=1 {
            if line_num == 0 && y == -1 || chr_num == 0 && x == -1 {
                continue;
            }
            let id = line_len 
                * (line_num + y) as usize 
                + (chr_num + x) as usize;
            if let Some(chr) = res_chars.get(id) {
                if !chr.is_ascii_digit() && *chr != '.' {
                    return true;
                }
            }
        }
    }
    false
}

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day03.txt")?;
    let mut sum = 0;
    let res_chars: Vec<char> = res.chars()
        .filter(|x| *x != '\n')
        .collect();
    for (line_num, line) in res.lines().enumerate() {
        let mut parsing_num = false;
        let mut num = String::new();
        let mut neighbor = false;
        for (chr_num, chr) in line.chars().enumerate() {
            if chr.is_ascii_digit() {
                parsing_num = true;
                num.push(chr);
                if !neighbor {
                    neighbor = check_neighbors(line.len(), line_num as isize, chr_num as isize, &res_chars);
                }
            } else {
                if parsing_num {
                    parsing_num = false;
                    if neighbor {
                        neighbor = false;
                        sum += num.parse::<i32>().unwrap();
                    }
                    num.clear();
                }
            }
        }
        if parsing_num && neighbor {
            sum += num.parse::<i32>().unwrap();
        }
    }
    println!("the sum for part 1 is {sum}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day03.txt")?;
    let mut sum = 0;
    let res_chars: Vec<char> = res.chars()
        .filter(|x| *x != '\n')
        .collect();
    for (line_num, line) in res.lines().enumerate() {
        for (chr_num, chr) in line.chars().enumerate() {
            if chr == '*' {
                let numbers = check_numbers(line.len(), line_num as isize, chr_num, &res_chars);
                if numbers.len() == 2 {
                    sum += numbers.iter().fold(1, |prod, next| prod * next);
                }
            }
        }
    }
    println!("the sum for part 2 is {sum}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
