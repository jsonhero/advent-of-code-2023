use std::{fs, collections::HashMap};


/*

Complexity: O(n)

Why?

We have to iterate through every character in the line, since the left/right number
could be at the very start of end of the string.

 */

fn problem_a() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut total: i32 = 0;

    let mut lines = input.lines();
    loop {
        if let Some(line) = lines.next() {
            let mut left_char: Option<char> = None;
            let mut right_char: Option<char> = None;

            for index in 0..line.len() {

                let left = line.chars().nth(index).unwrap();
                let right = line.chars().nth(line.len() - 1 - index).unwrap();

                if left.is_numeric() && left_char.is_none() {
                    left_char = Some(left);
                }

                if right.is_numeric() && right_char.is_none() {
                    right_char = Some(right);
                }

                if left_char.is_some() && right_char.is_some() {
                    break;
                }
            }

            if left_char.is_some() {
                total += (left_char.unwrap().to_string() + &right_char.unwrap().to_string()).parse::<i32>().unwrap()
            }

        } else {
            break;
        }
    }

    println!("Problem-A - total: {}", total);
}


#[derive(Debug)]
struct TreeNode {
    children: HashMap<char, TreeNode>,
    is_end_of_word: bool,
    value: Option<i32>
}

impl TreeNode {
    fn new(value: Option<i32>) -> TreeNode {
        TreeNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: value
        }
    }

    fn insert(&mut self, word: &str, value: i32) {
        let mut current_node = self;


        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(TreeNode::new(Some(value)));
        }

        current_node.is_end_of_word = true
    }
}

struct Number {
    word: &'static str,
    value: i32,
}

impl Number {
    fn new(word: &'static str, value: i32) -> Self {
        Number {
            word,
            value
        }
    }
}

// Needlessly overcomplicated this problem by adding a precompute map to improve time for words to O(1)-ish, but I'm just here for fun learning Rust
fn problem_b() {
    let mut root = TreeNode::new(None);

    let numbers: [Number; 18] = [
        Number::new("one", 1),
        Number::new("two", 2),
        Number::new("three", 3),
        Number::new("four", 4),
        Number::new("five", 5),
        Number::new("six", 6),
        Number::new("seven", 7),
        Number::new("eight", 8),
        Number::new("nine", 9),
        Number::new("1", 1),
        Number::new("2", 2),
        Number::new("3", 3),
        Number::new("4", 4),
        Number::new("5", 5),
        Number::new("6", 6),
        Number::new("7", 7),
        Number::new("8", 8),
        Number::new("9", 9),
    ];


    // setup
    for num in numbers {
        root.insert(num.word, num.value);
        if num.word.len() > 1 {
            let reversed = num.word.chars().rev().collect::<String>();
            root.insert(&reversed, num.value);
        }
    }



    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut total = 0;

    let mut lines = input.lines();
    loop {
        if let Some(line) = lines.next() {
            let mut word_node: Option<&TreeNode> = None;
            let mut index = 0;
            let mut word_index = index;


            'outer: loop {
                let outer_ch = line.chars().nth(index);                
                match outer_ch {
                    None => break 'outer,
                    Some(ch) => {                        
                        match word_node {
                            None => {
                                if root.children.contains_key(&ch) {
                                    word_index = index;
                                    word_node = root.children.get(&ch);
                                }
                            },
                            Some(node) => {
                                if node.children.contains_key(&ch) {
                                    word_node = node.children.get(&ch);
                                } else {
                                    index = word_index;
                                    word_node = None;
                                }
                            }
                        }
                    }
                }

                match word_node {
                    None => {},
                    Some(node) => {
                        if node.is_end_of_word {
                            total += node.value.unwrap() * 10;
                            break 'outer;
                        }
                    }
                }

                index += 1;
            }
            
            word_node = None;
            index = line.len() - 1;
            word_index = index;

            'outer: loop {
                let outer_ch = line.chars().nth(index);

                match outer_ch {
                    None => break 'outer,
                    Some(ch) => {                        
                        match word_node {
                            None => {
                                if root.children.contains_key(&ch) {
                                    word_index = index;
                                    word_node = root.children.get(&ch);
                                }
                            },
                            Some(node) => {
                                if node.children.contains_key(&ch) {
                                    word_node = node.children.get(&ch);
                                } else {
                                    index = word_index;
                                    word_node = None;
                                }
                            }
                        }
                    }
                }

                match word_node {
                    None => {},
                    Some(node) => {
                        if node.is_end_of_word {
                            total += node.value.unwrap();
                            break 'outer;
                        }
                    }
                }

                if index > 0  {
                    index -= 1;
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    println!("Problem-B - total: {}", total);

}

fn main() {
    problem_a();
    problem_b();
}

