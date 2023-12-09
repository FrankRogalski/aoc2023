use std::fs;
use std::error::Error;

fn part1() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day07.txt")?;
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error + 'static>> {
    let res = fs::read_to_string("resources/day07.txt")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    part1()?;
    part2()?;
    Ok(())
}
