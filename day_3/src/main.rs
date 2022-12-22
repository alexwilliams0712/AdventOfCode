use std::{char, fs};

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn find_repeated_item(all_items: &str) -> char {
    let length = all_items.chars().count();
    let (compartment1, compartment2) = all_items.split_at(length / 2);
    let mut char_value: char = char::default();
    for i in compartment1.chars() {
        if compartment2.contains(i) {
            char_value = i;
            break;
        }
    }
    char_value
}

fn find_group_badge(groups: &[&str]) -> char {
    let mut char_value: char = char::default();
    for i in groups[0].chars() {
        if groups[1].contains(i) && groups[2].contains(i) {
            char_value = i;
            break;
        }
    }

    char_value
}

fn find_priority(common_item: char) -> u32 {
    let lowercase_priority = ('a'..='z').into_iter().collect::<Vec<char>>();
    let uppercase_priority = ('A'..='Z').into_iter().collect::<Vec<char>>();

    let priority: u32;

    if common_item.is_uppercase() {
        priority = 27
            + uppercase_priority
                .iter()
                .position(|&r| r == common_item)
                .unwrap() as u32;
    } else {
        priority = 1 + lowercase_priority
            .iter()
            .position(|&r| r == common_item)
            .unwrap() as u32;
    }

    priority
}

fn main() {
    let contents = get_data();
    let rucksacks: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    let mut priority_sum = 0;
    for sack in &rucksacks {
        let common_item = find_repeated_item(sack);
        let common_item_priority = find_priority(common_item);
        priority_sum += common_item_priority
    }

    println!("Solution to part 1: {:?}", priority_sum);

    let mut priority_sum = 0;
    for group in rucksacks.chunks(3) {
        let shared_item = find_group_badge(group);
        priority_sum += find_priority(shared_item);
    }
    println!("Solution to part 2: {:?}", priority_sum);
}
