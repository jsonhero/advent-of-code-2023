use std::fs;


const SYMBOLS: [char; 10] = ['*', '+', '#', '-', '&', '/', '@', '$', '%', '='];
const POSITIONS: [[i32; 2]; 8] = [
    // left
    [0, 1],
    // right
    [0, -1],
    // top
    [-1, 0],
    // top left
    [-1, -1],
    // top right
    [-1, 1],
    // bottom
    [1, 0],
    // bottom left
    [1, -1],
    // bottom right
    [1, 1],
];


fn has_adjacent(matrix: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> bool {

    for pos in POSITIONS {
        let pos_row_idx = row_idx as i32 + pos[0];
        let possible_row = matrix.get(pos_row_idx as usize);
        
        if let Some(row) = possible_row {
            let pos_col_idx = col_idx as i32 + pos[1];
            let possible_column = row.get(pos_col_idx as usize);

            if let Some(ch) = possible_column {
                if SYMBOLS.contains(ch) {
                    return true
                }
            }
        }
    }
    return false
}

fn problem_a() {

    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut lines = input.lines();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    loop {
        if let Some(line) = lines.next() {

            let mut vec : Vec<char> = Vec::new();
            for ch in line.chars() {
                vec.push(ch);
            }
            matrix.push(vec);

        } else {
            break;
        }
    }

    let mut sum = 0;
    
    for (row_idx, row_vec) in matrix.iter().enumerate() {

        let mut num_str = String::new();
        let mut is_part_num = false;

        for (col_idx, ch) in row_vec.iter().enumerate() {
            if ch.is_numeric() {            
                if !is_part_num {
                    is_part_num = has_adjacent(&matrix, row_idx, col_idx);
                }
                num_str.push(*ch);
            }
            
            // End of number condition
            if num_str.len() > 0 && (col_idx == row_vec.len() - 1  || !ch.is_numeric()) {

                if is_part_num {
                    sum += num_str.parse::<i32>().unwrap();
                }
                // total
                num_str = String::new();
                is_part_num = false;
            }
        }
    }




    println!("Problem A: {}", sum);
}

fn get_part_number_range(matrix: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> [usize; 2] {
    let row = matrix.get(row_idx).unwrap();

    let mut range: [usize; 2] = [0, row.len()];

    // left
    for idx in (0..col_idx).rev() {
        let ch = row.get(idx).unwrap();

        if !ch.is_numeric() {
            range[0] = idx + 1;
            break;
        }
    }

    // right
    for idx in col_idx..row.len() {
        let ch = row.get(idx).unwrap();

        if !ch.is_numeric() {
           range[1] = idx;
           break;
        }
    }

    return range;
}

fn get_part_range_number(row: &Vec<char>, range: [usize; 2]) -> i32 {
    let part_num_str: &String = &row[range[0]..range[1]].iter().collect::<String>();
    return part_num_str.parse::<i32>().unwrap();
}

fn adjacent_gear_product(matrix: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> i32 {
    let mut part_1_range: Option<[usize; 2]> = None;
    let mut part_1_row_idx: usize = 0;

    for pos in POSITIONS {
        let pos_row_idx = (row_idx as i32 + pos[0]) as usize;
        let possible_row = matrix.get(pos_row_idx);
        
        if let Some(row) = possible_row {
            let pos_col_idx = (col_idx as i32 + pos[1]) as usize;
            let possible_column = row.get(pos_col_idx);

            if let Some(ch) = possible_column {
                if ch.is_numeric() {
                    if part_1_range.is_none() {
                        let part_range = get_part_number_range(matrix, pos_row_idx, pos_col_idx);
                        part_1_range = Some(part_range);
                        part_1_row_idx = pos_row_idx;                    
                    } 
                    // make sure range isn't overlapping for parts on the same row
                    else if !(pos_row_idx == part_1_row_idx && pos_col_idx >= part_1_range.unwrap()[0] && pos_col_idx <= part_1_range.unwrap()[1]) {
                        let part_range = get_part_number_range(matrix, pos_row_idx, pos_col_idx);
                        let part_1_num = get_part_range_number(matrix.get(part_1_row_idx).unwrap(), part_1_range.unwrap());
                        let part_2_num =  get_part_range_number(row, part_range);                    
                        return part_1_num * part_2_num;
                    }
                }
            }
        }
    }

    return 0;
}

fn problem_b() {

    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut lines = input.lines();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    loop {
        if let Some(line) = lines.next() {

            let mut vec : Vec<char> = Vec::new();
            for ch in line.chars() {
                vec.push(ch);
            }
            matrix.push(vec);

        } else {
            break;
        }
    }

    let mut sum = 0;

    for (row_idx, row_vec) in matrix.iter().enumerate() {
        for (col_idx, ch) in row_vec.iter().enumerate() {
            if ch.eq(&'*') {            
                sum += adjacent_gear_product(&matrix, row_idx, col_idx);
            }
        }
    }




    println!("Problem B: {}", sum);
}



fn main() {
    problem_a();
    problem_b();
}