use std::fs;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn find_all_sums(split: Vec<&str>) -> Vec<u32> {
    let mut elf_calorie_sum: Vec<u32> = vec![];
    for s in split {
        let mut cal_counter: u32 = 0;
        for cc in s.split("\n") {
            let num: u32 = cc.parse().unwrap();
            cal_counter += num
        }
        elf_calorie_sum.push(cal_counter);
    }
    elf_calorie_sum.sort();
    elf_calorie_sum.reverse();
    elf_calorie_sum
}

fn main() {
    let contents = get_data();
    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let elf_calorie_sum = find_all_sums(split);
    let max_val = elf_calorie_sum[0];
    println!("Part 1 solution: {:?}", max_val);

    let top_three_sum: u32 = elf_calorie_sum[0..3].to_vec().iter().sum();
    println!("Part 2 solution: {:?}", top_three_sum);
}
