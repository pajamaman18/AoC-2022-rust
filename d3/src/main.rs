use std::char::from_u32;
use std::fs;

fn main() {
    let file_path = "/home/pjman/personal-projects/AoC-2022-rust/d3/input.txt";
    let contents = fs::read_to_string(file_path).expect("no file found");
    let split = contents.split('\n');
    let bag_vec = split.collect::<Vec<&str>>();
    // println!("{:?}",bag_vec);
    let mut vec: Vec<&str> = Vec::new();
    for item in &bag_vec{
        let half = item.len()/2;
        let (fst,snd) = item.split_at(half);
        vec.push(fst);
        vec.push(snd);
    }
    // println!("{:?}",vec);
    let into_64_bit_vector: Vec<u64> = vec.into_iter().map(|string| to_64_bit(string)).collect();
    // println!("{:?}",into_64_bit_vector);
    let mut duplicate_compartment_item: Vec<u64> = Vec::new();
    for i in (0..into_64_bit_vector.len()).step_by(2){
        let bitwise_and: u64 = into_64_bit_vector[i]&into_64_bit_vector[i+1];
        duplicate_compartment_item.push(bitwise_and);
    }
    let mut sum = 0;
    for thing in &duplicate_compartment_item {
        // println!("{}",get_index_from(thing));
        sum += get_index_from(*thing);
    }
    println!("part1: {}",sum);

    let into_64_bit_vector_badge: Vec<u64> = bag_vec.into_iter().map(|string| to_64_bit(string)).collect();


    let mut badge_check: Vec<u64> = Vec::new();
    for i in (0..into_64_bit_vector_badge.len()-1).step_by(3){
        let bitwise_and: u64 = into_64_bit_vector_badge[i]& into_64_bit_vector_badge[i+1]& into_64_bit_vector_badge[i+2];
        badge_check.push(bitwise_and);
    }
    sum = 0;
    for thing in badge_check{
        // println!("{}",get_index_from(thing));
        sum += get_index_from(thing);
    }
    println!("part2: {}",sum);

}

fn get_index_from(integer: u64) -> u32{
    for i in 1..60{
        let mut test = 0b0;
        test |= 1 << i;
        if integer&test != 0{
            return i+1;
        }
    }
    return 0;
}

fn to_64_bit(item: &str) -> u64{
    let mut bitrep: u64 = 0b00;
    for c in item.chars() {
        let character_shift = to_bit(c);
        bitrep |= 1 << character_shift;
    }
    return bitrep;
    // println!("{:64b}", bitrep);
}

fn to_char(i: u32) -> char{
    return if i < 27{
        let t = i + 96;
        from_u32(t).unwrap()
    }else{
        let t = i + 39;
        from_u32(t).unwrap()
    }
}

fn to_bit(c: char) -> u32{
    let int = c as u32;
    return if int < 97 {
        int - 39
    } else {
        int - 96 -1
    }
}