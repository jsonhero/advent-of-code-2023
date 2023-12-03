use std::fs;
use regex::Regex;

fn problem_a() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut lines = input.lines();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total = 0;

    loop {
        if let Some(line) = lines.next() {
            let mut game_split = line.split(":");

            let game_id = game_split.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();

            let pattern = Regex::new(r"(\d+)\s+(\w+)").unwrap();

            let game_sets = game_split.nth(0).unwrap().split(";");

            let mut valid_game = true;
            
            for game_set in game_sets {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;

                for captures in pattern.captures_iter(game_set) {
                    if let (Some(number), Some(color)) = (captures.get(1), captures.get(2)) {
                        if color.as_str().eq("green") {
                            green += number.as_str().parse::<i32>().unwrap();
                        }
                        if color.as_str().eq("red") {
                            red += number.as_str().parse::<i32>().unwrap();
                        }
                        if color.as_str().eq("blue") {
                            blue += number.as_str().parse::<i32>().unwrap();
                        }
                    }  
                }

                if red > max_red || green > max_green || blue > max_blue {
                    valid_game = false;
                    break;
                }
            }

            if valid_game {
                total += game_id;
            }

        } else {
            break;
        }
    }

    println!("Problem A: {}", total);
}

fn problem_b() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut lines = input.lines();

    let mut total = 0;

    loop {
        if let Some(line) = lines.next() {
            let mut game_split = line.split(":");

            let game_id = game_split.nth(0).unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();

            let pattern = Regex::new(r"(\d+)\s+(\w+)").unwrap();

            let game_sets = game_split.nth(0).unwrap().split(";");

            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;
            
            for game_set in game_sets {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;

                for captures in pattern.captures_iter(game_set) {
                    if let (Some(number), Some(color)) = (captures.get(1), captures.get(2)) {
                        if color.as_str().eq("green") {
                            green += number.as_str().parse::<i32>().unwrap();
                        }
                        if color.as_str().eq("red") {
                            red += number.as_str().parse::<i32>().unwrap();
                        }
                        if color.as_str().eq("blue") {
                            blue += number.as_str().parse::<i32>().unwrap();
                        }
                    }  
                }

                if red > max_red {
                    max_red = red;
                }
                if blue > max_blue {
                    max_blue = blue;
                }
                if green > max_green {
                    max_green = green;
                }
            }

            total += max_red * max_green * max_blue;

        } else {
            break;
        }
    }

    println!("Problem B: {}", total);
}
fn main() {
    problem_a();
    problem_b();
}

