
use std::cmp::min;

 struct Race {
    time: u64,
    record_distance: u64
 }

static PROBLEM_A_RACES: [Race; 4] = [Race {
    time: 59,
    record_distance: 597,
},Race {
    time: 79,
    record_distance: 1234,
},Race {
    time: 65,
    record_distance: 1032,
},Race {
    time: 75,
    record_distance: 1328,
}];

static PROBLEM_B_RACES: [Race; 1] = [Race {
    time: 59796575,
    record_distance: 597123410321328,
}];

fn max_distance(duration: u64, max_time: u64) -> u64 {
    let remaining_time = max_time - duration;
    let max_dist = duration * remaining_time;
    max_dist
}



fn problem_a() {
    let mut total = 1;
    for race in PROBLEM_A_RACES.iter() {

        let mut ways = 0;
        
        for n in 1..=race.time {
            if max_distance(n, race.time) > race.record_distance {
                ways += 1;
            }
        }

        total *= ways;
    }
    println!("Problem A: {}", total);
}


// could use quadratic formula here instead to make this easier..
fn problem_b() {
    for race in PROBLEM_B_RACES.iter() {
        let mut ways = 0;
    
        for n in 1..=race.time {
            if max_distance(n, race.time) > race.record_distance {
                ways += 1;
            }
        }
        println!("Problem B: {}", ways);
    }
}

fn main() {
    problem_a();
    problem_b()
}

