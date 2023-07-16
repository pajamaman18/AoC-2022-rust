use std::fs;

fn main() {
    let file_path = "/home/pjman/personal-projects/AoC-2022-rust/d4/input.txt";
    let contents = fs::read_to_string(file_path).expect("no file found");
    let split = contents.split('\n');
    let bag_vec = split.collect::<Vec<&str>>();
    // println!("{:?}",bag_vec);
    let mut formatted_ids: Vec<&str> = Vec::new();
    let mut total_contains = 0;
    let mut total_overlap = bag_vec.len()-1;
    for item in bag_vec{
        if item != "" {
            let split: Vec<&str> = item.split(',').collect();
            let fst: Vec<&str> = split[0].split('-').collect::<Vec<&str>>();
            let snd: Vec<&str> = split[1].split('-').collect::<Vec<&str>>();
            let mut new_fst: Vec<i32> = Vec::new();
            let mut new_snd: Vec<i32> = Vec::new();
            for i in 0..2 {
                new_fst.push(fst[i].parse::<i32>().unwrap());
                new_snd.push(snd[i].parse::<i32>().unwrap());
            }
            if (new_fst[0] <= new_snd[0] && new_fst[1] >= new_snd[1]) ||  (new_fst[0] >= new_snd[0] && new_fst[1] <= new_snd[1]){
                total_contains += 1;
            }
            if new_fst[1] < new_snd[0] || new_snd[1] < new_fst[0] {
                total_overlap -= 1;
            }
            // println!("{:?}",(new_fst,new_snd));
        }
    }

    println!("part1: {:?}",total_contains);
    println!("part2: {:?}",total_overlap);


}
