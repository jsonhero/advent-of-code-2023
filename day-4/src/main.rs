use std::fs;

fn problem_a() {
    let input = fs::read_to_string("src/input.txt").unwrap();    
    let mut total = 0;

    for line in input.lines() {
        let card_split: Vec<&str> = line.split(":").collect();
        let number_split: Vec<&str> = card_split[1].split("|").collect();

        let winning_numbers: Vec<i32> = number_split[0].split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        
        let guess_numbers: Vec<i32> = number_split[1].split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();

        let sum = guess_numbers.iter().filter(|n| winning_numbers.contains(n)).fold(0, |sum, _n| {
            if sum == 0 {
                return 1
            }
            return sum * 2
        });

        total += sum;
    }

    println!("Problem A: {}", total);
}


fn problem_b() {
    let input = fs::read_to_string("src/input.txt").unwrap();    
    let mut total = 0;
    let lines: Vec<&str> = input.lines().collect();

    let mut copies: Vec<i32> = vec![1; lines.len()];

    for (line_idx, line) in lines.iter().enumerate() {
        let card_split: Vec<&str> = line.split(":").collect();
        let number_split: Vec<&str> = card_split[1].split("|").collect();

        let winning_numbers: Vec<i32> = number_split[0].split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        let guess_numbers: Vec<i32> = number_split[1].split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        let matching_numbers: Vec<&i32> = guess_numbers.iter().filter(|n| winning_numbers.contains(n)).collect();

        for idx in 1..=matching_numbers.len() {
            copies[line_idx + idx] += copies[line_idx];
        }

        total += copies[line_idx];
    }

    println!("Problem B: {}", total);
}

fn main() {
    problem_a();
    problem_b();
}

