use std::{fs, collections::HashMap};
use std::rc::Rc;


static STRENGTH_ORDER: &'static[ char; 13] = &[
    'A',
    'K',
    'Q',
    'J',
    'T',
    '9',
    '8',
    '7',
    '6',
    '5',
    '4',
    '3',
    '2'
];
type StrengthMap = HashMap<&'static char, i16>;

fn create_strength_map(joker: bool) -> StrengthMap {
    let mut strength_map: StrengthMap = HashMap::new();

    for (idx, ch) in STRENGTH_ORDER.iter().enumerate() {
        if joker && ch.eq(&'J') {
            strength_map.insert(ch, STRENGTH_ORDER.len() as i16);
        } else {
            strength_map.insert(ch, idx as i16);
        }
    }

    return strength_map;
}


#[derive(Debug, PartialEq)] // Derive the PartialEq trait for automatic equality checks
enum HandType {
    FiveKind = 0,
    FourKind = 1,
    FullHouse = 2,
    ThreeKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6
}

enum HandEntry {
    String,
    Map(HashMap<&'static str, Rc<HandEntry>>),
}

type HandCardMap = HashMap<char, i16>;

fn get_hand_card_map(hand: &str, joker: bool) -> HandCardMap {
    let mut card_map: HandCardMap = HashMap::new();

    for ch in hand.chars() {
        card_map.entry(ch).and_modify(|n| {
            *n += 1;
        }).or_insert(1);
    }
    
    if joker && card_map.contains_key(&'J') && card_map.len() > 1 {
        let joker_count = *card_map.get(&'J').unwrap();
        card_map.remove(&'J');
        let mut highest_card: Option<char> = None;
        
        for (card, count) in card_map.iter() {
            let mut highest_count: &i16 = &0;

            if highest_card.is_some() {
                highest_count = card_map.get(&highest_card.unwrap()).unwrap();
            }

            if count > highest_count {
                highest_card = Some(*card);
            }
        }

        if let Some(card) = &highest_card {
            card_map.entry(*card).and_modify(|c| {
                *c += joker_count;
            });
        }
    }

    return card_map
}

fn get_hand_type(card_map: HandCardMap) -> HandType {
    let mut pair_fallback: Option<HandType> = None;
    for n in card_map.values() {
        if n.eq(&5) {
            return HandType::FiveKind
        } else if n.eq(&4) {
            return HandType::FourKind
        } else if n.eq(&3) {
            if pair_fallback.is_some() {
                return HandType::FullHouse
            } else {
                pair_fallback = Some(HandType::ThreeKind);
            }
        } else if n.eq(&2) {
            if let Some(fallback) = &pair_fallback {
                if *fallback == HandType::ThreeKind {
                    return HandType::FullHouse;
                } else if *fallback == HandType::OnePair {
                    return HandType::TwoPair
                }
            } else {
                pair_fallback = Some(HandType::OnePair);
            }
        }
    }

    match pair_fallback {
        None => {
            return HandType::HighCard
        },
        Some(fallback) => {
            return fallback
        }
    }
}

fn run(joker: bool) -> i64 {
    let strength_map = create_strength_map(joker);
    let input = fs::read_to_string("src/input.txt").unwrap();    
    let mut lines: Vec<Vec<&str>> = input.lines().map(|l: &str| l.split_whitespace().collect()).collect();


    // Sort here is pretty costly, could use deep card map instead to avoid a sort or could precompute each value in the array
    lines.sort_by(|a, b| {
        let hand_1 = a[0];
        let hand_2 = b[0];

        let hand_type_1 = get_hand_type(get_hand_card_map(hand_1, joker));
        let hand_type_2 = get_hand_type(get_hand_card_map(hand_2, joker));

        if hand_type_1 == hand_type_2 {
            let hand_len = a[0].len();

            for i in 0..hand_len {
                let h1_str = strength_map.get(&hand_1.chars().nth(i).unwrap()).unwrap();
                let h2_str = strength_map.get(&hand_2.chars().nth(i).unwrap()).unwrap();

                if !h1_str.eq(h2_str) {
                    return h2_str.cmp(h1_str);
                }
            }
        }

        return (hand_type_2 as i16).cmp(&(hand_type_1 as i16));
    });


    let mut sum: i64 = 0;

    for (idx, line) in lines.iter().enumerate() {
        let bet = line[1].parse::<i64>().unwrap();

        sum += bet * (idx + 1) as i64;
    }

    return sum;
}

fn problem_a() {
    println!("Problem A: {}", run(false));
}

fn problem_b() {
    println!("Problem B: {}", run(true));

}


fn main() {
    problem_a();
    problem_b()
}
