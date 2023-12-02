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
            println!("Got line: {}", line);

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

    println!("Total: {}", total);
}



/*

example input:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

brainstorm:

static map of numbers 1-9

as iterate through each line,
then add char to string
check if char combo matches any of the numbers in the map

Map:
one
two
three
four
five
six
seven
eight
nine

ie.
oone2

Could do look ahead each time first char matches?


O(n * maplength)
Check if all chars after match?
t = two, three
f = four, five

Preprocessed token map?

f {
    i {
        v {
            e
        }
    }
    o {
        u {
            r
        }
        
    }
}

Each iteration of a letter brings you one deeper in the map depending on next letter

This leads to O(1) lookup speed well iterating, but requires some setup ahead of time.



Brute force method = O(n * log n) + O(1)

Create map of word numbers.
Iterate through each letter, then iterate through every combonation of next letters from that starting letter, for every combo then lookup in the map if there's a matching word


*/
#[derive(Debug)]
struct TreeNode {
    children: HashMap<char, TreeNode>,
    is_end_of_word: bool,
    value: Option<char>
}


impl TreeNode {
    fn new(value: Option<char>) -> TreeNode {
        TreeNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: value
        }
    }

    fn insert(&mut self, word: &str, value: char) {
        let mut current_node = self;


        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(TreeNode::new(Some(value)));
        }

        current_node.is_end_of_word = true
    }
}

fn problem_b() {
    let mut root = TreeNode::new(None);

    // setup
    root.insert("one", '1');
    root.insert("two", '2');
    root.insert("three", '3');
    root.insert("four", '4');
    root.insert("five", '5');
    root.insert("six", '6');
    root.insert("seven", '7');
    root.insert("eight", '8');
    root.insert("nine", '9');



    let input = fs::read_to_string("src/input.txt").unwrap();
    let mut total: i32 = 0;

    let mut lines = input.lines();
    loop {
        if let Some(line) = lines.next() {

            let mut left_char: Option<char> = None;
            let mut right_char: Option<char> = None;

            let mut word_node: Option<&TreeNode> = None;

            println!("{}", line);
            for index in 0..line.len() {

                let ch = line.chars().nth(index).unwrap();
                // println!("{}", ch);

                if ch.is_numeric() {

                    if left_char.is_none() {
                        left_char = Some(ch);
                    }
                    right_char = Some(ch);
                }
                
                if word_node.is_some() && word_node.unwrap().children.contains_key(&ch) {
                    word_node = word_node.unwrap().children.get(&ch);
                } else if root.children.contains_key(&ch) {
                    word_node = root.children.get(&ch);
                } else {
                    word_node = None;
                }

                if word_node.is_some() && word_node.unwrap().is_end_of_word {
                    if left_char.is_none() {
                        left_char = Some(word_node.unwrap().value.unwrap())
                    }
                    right_char = Some(word_node.unwrap().value.unwrap())
                }
            }

            if left_char.is_some() {
                println!("Actual: {}", left_char.unwrap().to_string() + &right_char.unwrap().to_string());
                total += (left_char.unwrap().to_string() + &right_char.unwrap().to_string()).parse::<i32>().unwrap()
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
