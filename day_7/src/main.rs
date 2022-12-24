use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;

fn get_data() -> String {
    let file_path = "src/input.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Directory {
    name: String,
    size: u32,
    files: HashMap<String, u32>,
    subdirectories: HashMap<String, Directory>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            size: 0,
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        }
    }
    fn update_sizes(&mut self) {
        let mut remaining_subdirs = true;
        while remaining_subdirs {
            let mut sub_dir_sizes: Vec<u32> = vec![];
            for sub_dir in self.subdirectories.values() {
                sub_dir_sizes.push(sub_dir.size)
            }
            if self.subdirectories.len() == 0 {
                self.size = self.files.values().sum();
                remaining_subdirs = false
            } else if sub_dir_sizes.iter().all(|x| *x != 0) {
                self.size = self.files.values().sum::<u32>() + sub_dir_sizes.iter().sum::<u32>();
                remaining_subdirs = false
            } else {
                for (_, sub_dir) in self.subdirectories.iter_mut() {
                    sub_dir.update_sizes()
                }
            }
        }
    }

    fn add_subdirectories(&mut self, mut current_pwd: VecDeque<&str>) {
        let name = current_pwd.pop_front().unwrap();
        if !self.subdirectories.contains_key(name) {
            self.subdirectories
                .insert(name.to_string(), Directory::new(name));
        }
        if current_pwd.len() > 0 {
            self.subdirectories
                .get_mut(name)
                .unwrap()
                .add_subdirectories(current_pwd);
        }
    }

    fn add_files(&mut self, mut current_pwd: VecDeque<&str>, file_data: &str) {
        if current_pwd.len() == 0 {
            let mut iter = file_data.split_whitespace();
            let file_size: u32 = iter.next().unwrap().parse().unwrap();
            let filename: String = iter.next().unwrap().to_string();
            self.files.insert(filename, file_size);
        } else {
            let next_dir = current_pwd.pop_front().unwrap();
            self.subdirectories
                .get_mut(next_dir)
                .unwrap()
                .add_files(current_pwd, file_data)
        }
    }

    fn find_dir_sizes(&mut self, mut dirs: HashMap<String, u32>) -> HashMap<String, u32> {
        dirs.insert(self.name.clone(), self.size);
        for (_, sub_dir) in self.subdirectories.iter_mut() {
            dirs = sub_dir.find_dir_sizes(dirs.clone())
        }
        dirs
    }
}

fn get_directories(cmd_arr: Vec<&str>) -> Directory {
    let mut directories = Directory::new("/");
    let mut current_pwd_arr: VecDeque<&str> = VecDeque::new();
    for ln in cmd_arr {
        if ln.chars().nth(0).unwrap() == '$' {
            let mut cmd_iter = ln.split_whitespace();
            let cmd = cmd_iter.nth(1).unwrap();
            if cmd == "cd" {
                let arg = cmd_iter.next().unwrap();
                if arg == "/" {
                    current_pwd_arr.clear();
                } else if arg == ".." {
                    current_pwd_arr.pop_back();
                } else {
                    current_pwd_arr.push_back(arg);
                }

                if current_pwd_arr.len() > 0 {
                    directories.add_subdirectories(current_pwd_arr.clone());
                }
            }
        } else if !ln.starts_with("dir ") {
            directories.add_files(current_pwd_arr.clone(), ln);
        }
    }
    directories.update_sizes();
    directories
}

fn get_small_directories(mut directories: Directory, max_size: u32) -> u32 {
    let all_dirs = directories.find_dir_sizes(HashMap::new());
    let mut sum_of_small: u32 = 0;
    for (name, size) in all_dirs {
        if size <= max_size {
            sum_of_small += size;
            println!("{}: {}", name, size)
        }
    }
    sum_of_small
}

fn main() {
    let max_size = 100_000;
    let contents = get_data();
    let cmd_arr: Vec<&str> = contents.split("\n").collect();
    let directories = get_directories(cmd_arr);
    let _serialized_dir = serde_json::to_string_pretty(&directories).unwrap();
    let small_directories = get_small_directories(directories, max_size);
    println!("Solution to part 1: {:?}", small_directories);
    println!("{}", _serialized_dir);
}
