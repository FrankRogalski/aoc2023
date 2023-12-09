use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day02.txt")?;
    let mut max = HashMap::new();
    max.insert("red", 12);
    max.insert("green", 13);
    max.insert("blue", 14);
    let mut sum = 0;
    for line in res.lines() {
        let (id, games) = line.split_once(':').unwrap();
        let id = id.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        sum += id;
        'game_loop: for game in games.split(';') {
            let dice = game.split(',');
            for die in dice.map(|x| x.trim()) {
                let (amount, color) = die.split_once(' ').unwrap();
                if amount.parse::<i32>().unwrap() > *max.get(color).unwrap() {
                    sum -= id;
                    break 'game_loop;
                }
            }
        }
    }
    println!("the answer to part 1 is {sum}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day02.txt")?;
    let mut max = HashMap::new();
    let mut sum = 0;
    for line in res.lines() {
        let games = line.split_once(':').unwrap().1;
        max.clear();
        for game in games.split(';') {
            let dice = game.split(',');
            for die in dice.map(|x| x.trim()) {
                let (amount, color) = die.split_once(' ').unwrap();
                let amount = amount.parse::<i32>().unwrap();
                max.entry(color).and_modify(|x| {
                    if *x < amount {
                        *x = amount;
                    }
                }).or_insert(amount);
            }
        }
        sum += max.values().fold(1, |prod, next| prod * next);
    }
    println!("the answer to part 2 is {sum}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
