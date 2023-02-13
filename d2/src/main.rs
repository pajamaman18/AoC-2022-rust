use std::fs;


fn main(){
    let file_path = "/home/pjman/personal-projects/AoC-2022-rust/day2/input.txt";
    println!("file path:{}", file_path);
    let contents = fs::read_to_string(file_path).expect("no file found");
    let split = contents.split('\n');
    let vec = split.collect::<Vec<&str>>();
    let mut simp: Vec<(i32, i32)> = Vec::new();
    for fight in vec{
        if fight.to_string() != "" {
            let m : (i32,i32) = (makedigit(fight.to_string().chars().nth(0).unwrap()),makedigit(fight.to_string().chars().nth(2).unwrap()));
            simp.push(m);
        }
    }
    println!("part1:");
    let mut outcome = 0;
    let mut outcome2 = 0;
    for item in simp{
        let fst = item.0;
        let snd = item.1;
        if fst == snd{
            outcome += 3 + fst;
        }else if fst + 1 == snd {
            outcome += 6 + snd;
        }else if fst == 3 && snd == 1{
            outcome += 6 + snd;
        }else{
            outcome += 0 + snd;
        }
        let v = match snd{
            1 => 0 + lose(lose(fst)),
            2 => 3 + fst,
            3 => 6 + lose(fst),
            _ => 0
        };
        outcome2 += v;
    }
    print!("{}",outcome);
    println!("part2:");
    print!("{}",outcome2);
    // println!("{:?}",simp)
}

fn lose(enemy: i32) -> i32{
    return if enemy < 3 {
        enemy + 1
    } else {
        1
    }
}

fn makedigit(c: char) -> i32{
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0
    }
}