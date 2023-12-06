use std::{fs, collections::HashMap};

const MAP_IDS: [&'static str; 7] = [
    "seed-to-soil", 
    "soil-to-fertilizer", 
    "fertilizer-to-water", 
    "water-to-light", 
    "light-to-temperature", 
    "temperature-to-humidity", 
    "humidity-to-location"
];

fn problem_a() {
    let input = fs::read_to_string("src/input.txt").unwrap();    
    let lines = input.lines();

    let mut seeds: Vec<i64>  = Vec::new();


    // Map<id, Vec[destination, source, range][]>
    let mut maps: HashMap<&str, Vec<Vec<i64>>> = HashMap::new();


    // setup
    let mut active_parsing_map: Option<&str> = None;
    for line in lines {
        if line.starts_with("seeds") {
            let seed_line: Vec<&str> = line.split("seeds: ").collect();
            seeds.extend(seed_line[1].split_whitespace().map(|n| n.parse::<i64>().unwrap()));
        }

        match active_parsing_map {
            None => {
                let found_map = MAP_IDS.iter().find(|&s| line.starts_with(s));
                if found_map.is_some() {
                    active_parsing_map = Some(found_map.unwrap());
                    maps.insert(found_map.unwrap(), vec![]);
                }
            },
            Some(map_id) => {
                let values: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

                if values.len() > 0 {
                    maps.entry(map_id).and_modify(|outer_vec| {
                        outer_vec.push(values);
                    });
                } else {
                    active_parsing_map = None;
                }
            }
        }
    }

    println!("{:?}", seeds);

    let mut lowest_loc: Option<i64> = None;

    for seed in seeds {
        let mut seed_loc = seed;

        for map_id in MAP_IDS {
            let map = maps.get(map_id).unwrap();

            for list in map {
                let range = list[2];
                let source = list[1];
            
                if seed_loc >= source && seed_loc < (source + range) {    
                    let diff = seed_loc - source;
                    let dest = list[0];
                    // println!("{} {}, {}, {} - {}", dest, source, range, seed_loc, dest + diff);
                    seed_loc = dest + diff;
                    break;
                }
            }
        }

        if lowest_loc.is_none() {
            lowest_loc = Some(seed_loc);
        } else if lowest_loc.unwrap().gt(&seed_loc) {
            lowest_loc = Some(seed_loc);
        }
    }

    println!("Problem A: {}", lowest_loc.unwrap());
}


/*
79,14 = 79-92

41-300

seed -> soil
50-51
52-99

soil -> fert
1: 35-36 





*/

fn process_ranges_map(ranges: Vec<Vec<i64>>, map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut next_ranges: Vec<Vec<i64>> = vec![];

    for range in ranges.iter() {
        let start = range[0];
        let end = range[1];
        println!("r: {}-{}", start, end);

        let saved_range_len = next_ranges.len();

        // For each list in the map check each range
        for list in map {
            let list_range = list[2];
            let source = list[1];
            let dest = list[0];


            let source_start = source;
            let source_end = source + list_range - 1;

            println!("list: {}-{}: {}", source_start, source_end, list_range);

            // starts in range, no cap
            if start >= source_start && end <= source_end {
                let next_start = dest + (start - source_start);
                let next_end = dest + (end - source_start);
                println!("     within: {}-{}", start, end);

                next_ranges.push(vec![next_start, next_end]);
            } 
            // starts high, cap highest
            else if start >= source_start && start <= source_end {
                let next_start = dest + (start - source_start);
                let next_end = dest + (source_end - source_start);
                println!("     start high: {}-{}", start, end);
                next_ranges.push(vec![next_start, next_end]);

                let reprocess_ranges = vec![vec![source_end + 1, end]];
                let mut next = process_ranges_map(reprocess_ranges, map);
                next_ranges.append(&mut next);
            } 
            // starts less, cap lowest
            else if start <= source_start && end >= source_start {
                let next_start = dest;
                let next_end = dest + (end - source_start);
                println!("     start low: {}-{}", start, end);
                next_ranges.push(vec![next_start, next_end]);

                let reprocess_ranges = vec![vec![start, source_start - 1]];
                let mut next = process_ranges_map(reprocess_ranges, map);
                next_ranges.append(&mut next);
            } 
            // full range, if range within then run middle part through that matches, and outer edges become ranges
            else if source_start > start && source_end < end {
                println!("     split: {}-{}", start, end);

                let inner_next_start = dest;
                let inner_next_end = dest + list_range;
                next_ranges.push(vec![inner_next_start, inner_next_end]);
                
                // Need to reprocess outer ranges
                let mut reprocess_ranges: Vec<Vec<i64>> = vec![];

                let left_next_start = source_end + 1;
                let left_next_end = end;
                reprocess_ranges.push(vec![left_next_start, left_next_end]);

                let right_next_start = start;
                let right_next_end = source_start - 1;
                reprocess_ranges.push(vec![right_next_start, right_next_end]);

                let mut next = process_ranges_map(reprocess_ranges, map);
                next_ranges.append(&mut next);
            }
        }

        if saved_range_len == next_ranges.len() {
            println!("     outer: {}-{}", start, end);
            next_ranges.push(range.clone());
        }
    }

    return next_ranges
}

fn problem_b() {
    let input = fs::read_to_string("src/input.txt").unwrap();    
    let lines = input.lines();

    let mut seeds: Vec<Vec<i64>>  = Vec::new();


    // Map<id, Vec[destination, source, range][]>
    let mut maps: HashMap<&str, Vec<Vec<i64>>> = HashMap::new();


    // setup
    let mut active_parsing_map: Option<&str> = None;
    for line in lines {
        if line.starts_with("seeds") {
            let seed_line: Vec<&str> = line.split("seeds: ").collect();
            let seed_range_line: Vec<i64> = seed_line[1].split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

            for index in 0..(seed_range_line.len() / 2) {
                let range_index = index * 2;    
                let start = seed_range_line[range_index];
                let end = start + seed_range_line[range_index + 1] - 1;
                let seed_range = vec![start, end];
                seeds.push(seed_range);
            }
            
        }

        match active_parsing_map {
            None => {
                let found_map = MAP_IDS.iter().find(|&s| line.starts_with(s));
                if found_map.is_some() {
                    active_parsing_map = Some(found_map.unwrap());
                    maps.insert(found_map.unwrap(), vec![]);
                }
            },
            Some(map_id) => {
                let values: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

                if values.len() > 0 {
                    maps.entry(map_id).and_modify(|outer_vec| {
                        outer_vec.push(values);
                    });
                } else {
                    active_parsing_map = None;
                }
            }
        }
    }

    let mut current_ranges: Vec<Vec<i64>> = seeds;

    for map_id in MAP_IDS {
        let map: &Vec<Vec<i64>> = maps.get(map_id).unwrap();

        let next_ranges = process_ranges_map(current_ranges, map);
        current_ranges = next_ranges;
        println!("----------");
        println!("{}: {:?}", map_id, current_ranges);
        println!("----------");
    }

    let mut lowest_loc: Option<i64> = None;

    for range in current_ranges {
        if lowest_loc.is_none() {
            lowest_loc = Some(range[0]);
        } else if range[0] < lowest_loc.unwrap() {
            lowest_loc = Some(range[0]);
        }
    }

    println!("Problem B: {}", lowest_loc.unwrap());
}


fn main() {
    problem_a();
    problem_b()
}
