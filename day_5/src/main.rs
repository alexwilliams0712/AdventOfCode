use std::collections::VecDeque;
use std::fs;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn build_ship_stacks(ship_stacks_str: &str) -> Vec<VecDeque<char>> {
    let num_stacks = ship_stacks_str
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();
    let rows = ship_stacks_str.split("\n").collect::<Vec<&str>>();
    let mut ship_stack: Vec<VecDeque<char>> = vec![];
    for j in 1..=num_stacks {
        let i = (((j - 1) * 4) + 1) as usize;
        ship_stack.push(VecDeque::new());
        for (row_num, row_str) in rows.iter().enumerate() {
            if row_num < num_stacks as usize {
                let val = row_str.chars().nth(i);
                if let Some(val) = val {
                    if val.is_alphabetic() {
                        ship_stack[(j - 1) as usize].push_back(val);
                    }
                }
            }
        }
    }
    ship_stack
}

fn get_single_moves(moves_str: &str) -> Vec<(u32, u32)> {
    let mut moves: Vec<(u32, u32)> = vec![];
    for next_move in moves_str.split("\n") {
        let mut iter = next_move.split_whitespace();
        let reps = iter.nth(1).expect("Reps failed").parse::<u32>().unwrap();
        let from_col = iter
            .nth(1)
            .expect("from col failed")
            .parse::<u32>()
            .unwrap()
            - 1;
        let to_col = iter.nth(1).expect("to col failed").parse::<u32>().unwrap() - 1;
        for _ in 0..reps {
            moves.push((from_col, to_col))
        }
    }
    moves
}

fn do_single_moves(
    moves: Vec<(u32, u32)>,
    mut ship_stacks: Vec<VecDeque<char>>,
) -> Vec<VecDeque<char>> {
    for single_move in moves {
        let element = ship_stacks[single_move.0 as usize]
            .pop_front()
            .expect("VecDeq empty!");
        ship_stacks[single_move.1 as usize].push_front(element);
    }
    ship_stacks
}

fn get_multiple_moves(moves_str: &str) -> Vec<(u32, u32, u32)> {
    let mut moves: Vec<(u32, u32, u32)> = vec![];
    for next_move in moves_str.split("\n") {
        let mut iter = next_move.split_whitespace();
        let reps = iter.nth(1).expect("Reps failed").parse::<u32>().unwrap();
        let from_col = iter
            .nth(1)
            .expect("from col failed")
            .parse::<u32>()
            .unwrap()
            - 1;
        let to_col = iter.nth(1).expect("to col failed").parse::<u32>().unwrap() - 1;
        moves.push((reps, from_col, to_col))
    }
    moves
}

fn do_multiple_moves(
    moves: Vec<(u32, u32, u32)>,
    mut ship_stacks: Vec<VecDeque<char>>,
) -> Vec<VecDeque<char>> {
    for single_move in moves {
        let mut temp_stack = VecDeque::new();
        for _ in 0..single_move.0 {
            let element = ship_stacks[single_move.1 as usize]
                .pop_front()
                .expect("VecDeq empty!");
            temp_stack.push_front(element)
        }
        for _ in 0..temp_stack.len() {
            let element = temp_stack.pop_front().unwrap();
            ship_stacks[single_move.2 as usize].push_front(element);
        }
    }
    ship_stacks
}

fn main() {
    let contents = get_data();
    let mut split = contents.split("\n\n");
    let ship_stacks_str = split.next().unwrap();
    let moves_str = split.next().unwrap();
    let mut ship_stacks = build_ship_stacks(ship_stacks_str);
    let single_moves = get_single_moves(&moves_str);
    ship_stacks = do_single_moves(single_moves, ship_stacks);
    let mut answer: String = "".to_string();
    for ship_stack in ship_stacks {
        answer.push(ship_stack[0])
    }
    println!("Solution to part 1: {:?}", answer);

    let mut ship_stacks = build_ship_stacks(ship_stacks_str);
    let multi_moves = get_multiple_moves(&moves_str);
    ship_stacks = do_multiple_moves(multi_moves, ship_stacks);
    let mut answer: String = "".to_string();
    for ship_stack in ship_stacks {
        answer.push(ship_stack[0])
    }
    println!("Solution to part 2: {:?}", answer);
}
