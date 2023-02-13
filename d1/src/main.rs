// use std::env;
use std::fs;

fn main() {
    let file_path = "/home/pjman/personal-projects/AoC-2022-rust/day1/input.txt";
    println!("file path:{}", file_path);
    let contents = fs::read_to_string(file_path).expect("no file found");
    let split = contents.split('\n');
    let vec = split.collect::<Vec<&str>>();
    let mut current = 0;
    let mut elves: Vec<i32> = Vec::new();
    for elf in vec{
        if elf != "\n" && elf != ""{
            let inted = elf.parse::<i32>().unwrap();
            current += inted;
        }else{
            elves.push(current);
            current = 0;
        }
    }
    elves.sort();
    print!("part1:");
    let mut sum: i32 = 0;
    for _i in 0..1{
        let highest = elves.pop().unwrap();
        sum += highest;
    }
    println!("{}",sum);
    print!("part2:");
    sum = 0;
    for _i in 0..3{
        let highest = elves.pop().unwrap();
        sum += highest;
    }
    println!("{}",sum)

}