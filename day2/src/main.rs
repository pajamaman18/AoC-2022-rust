use std::fs;

fn main(){
    let file_path = "/home/pjman/personal-projects/AoC-2022-rust/day1/input.txt";
    println!("file path:{}", file_path);
    let contents = fs::read_to_string(file_path).expect("no file found");
    let split = contents.split('\n');
    let vec = split.collect::<Vec<&str>>();
}