use core::str::Split;
use range_check::Check;
use std::fs;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn format_split(mut split: Split<&str>) -> (u32, u32) {
    (
        split.next().unwrap().parse().expect("Invalid str"),
        split.next().unwrap().parse().expect("Invalid str"),
    )
}

fn find_complete_overlaps(elves: ((u32, u32), (u32, u32))) -> bool {
    let mut response: bool = false;
    let ((first_elf_start, first_elf_end), (second_elf_start, second_elf_end)) = elves;
    if (first_elf_start <= second_elf_start && first_elf_end >= second_elf_end)
        || (first_elf_start >= second_elf_start && first_elf_end <= second_elf_end)
    {
        response = true;
    }

    response
}

fn find_partial_overlap(elves: ((u32, u32), (u32, u32))) -> bool {
    let ((first_elf_start, first_elf_end), (second_elf_start, second_elf_end)) = (
        (elves.0 .0 as i32, elves.0 .1 as i32),
        (elves.1 .0 as i32, elves.1 .1 as i32),
    );

    let response = match first_elf_start.check_range(second_elf_start..=second_elf_end) {
        Ok(_) => true,
        Err(_) => false,
    } || match second_elf_start.check_range(first_elf_start..=first_elf_end) {
        Ok(_) => true,
        Err(_) => false,
    };
    response
}

fn handle_elf_pair(elf_pair: &str) -> ((u32, u32), (u32, u32)) {
    let mut split = elf_pair.split(",");
    (
        format_split(split.next().unwrap().split("-")),
        format_split(split.next().unwrap().split("-")),
    )
}

fn main() {
    let contents = get_data();
    let elf_pairs: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut fully_contained_counter: u32 = 0;
    let mut fully_contained: bool;

    let mut partially_contained_counter: u32 = 0;
    let mut partially_contained: bool;

    for elf_pair in elf_pairs {
        let elves = handle_elf_pair(elf_pair);
        fully_contained = find_complete_overlaps(elves);
        if fully_contained {
            fully_contained_counter += 1;
        }

        partially_contained = find_partial_overlap(elves);
        if partially_contained {
            partially_contained_counter += 1;
        }
    }
    println!("Solution to part 1: {:?}", fully_contained_counter);
    println!("Solution to part 2: {:?}", partially_contained_counter);
}
