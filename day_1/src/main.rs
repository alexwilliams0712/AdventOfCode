use std::fs;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}


fn main() {
    let contents = get_data();

    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let mut elf_calorie_sum:Vec<u32> = vec![];
    for s in split {
        let mut cal_counter: u32 = 0;
        for cc in s.split("\n") {
            let num: u32 = cc.parse().unwrap();
            cal_counter += num
        }
        elf_calorie_sum.push(cal_counter);
    }

    let max_index = elf_calorie_sum.iter()
        .enumerate()
        .max_by_key(|(_, &x)| x)
        .unwrap()
        .0 + 1;
    println!("{:?}", max_index);
}
