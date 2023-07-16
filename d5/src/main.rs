use std::fs;

fn main() {
    let file_path = "/home/pjman/personal-projects/AoC-2022-rust/d5/input.txt";
    let contents = fs::read_to_string(file_path).expect("no file found");
    let split = contents.split('\n');
    let bag_vec = split.collect::<Vec<&str>>();
    let mut stacks = Vec::new();
    let mut moves = Vec::new();
    for item in 0..bag_vec.len(){
        if item < 8{
            stacks.push(bag_vec[item])
        }else{
            moves.push(bag_vec[item])
        }
    }
    let actual_moves = moves[2..].to_vec();
    println!("{:?}",actual_moves);

}