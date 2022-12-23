use std::collections::{HashSet, VecDeque};
use std::fs;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn process_stream(number_distinct_vals: u32, stream: &str) -> u32 {
    let mut pre_stream: VecDeque<char> = VecDeque::new();
    let mut first_val = 0;
    for (i, val) in stream.chars().enumerate() {
        if i >= number_distinct_vals as usize {
            let unique_set: HashSet<char> = pre_stream.iter().cloned().collect();
            if unique_set.len() == pre_stream.len() {
                first_val = i as u32;
                break;
            }
            pre_stream.pop_back();
        }
        pre_stream.push_front(val);
    }
    first_val
}

fn main() {
    let contents = get_data();
    let response = process_stream(4, contents.as_str());
    println!("Part 1 response: {:?}", response);

    let response = process_stream(14, contents.as_str());
    println!("Part 2 response: {:?}", response);
}
