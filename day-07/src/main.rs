use std::collections::HashMap;

use aoc_lib::input;

#[derive(Debug)]
struct Dir {
    children: Vec<String>,
    size: usize,
}

impl Dir {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            size: 0,
        }
    }

    fn calculate_size<'a>(
        &'a self,
        folders: &'a HashMap<String, Dir>,
        size_cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if self.children.len() == 0 {
            return self.size;
        }

        self.size
            + self
                .children
                .iter()
                .map(|child| {
                    let child = child.as_str();
                    if size_cache.contains_key(child) {
                        return *size_cache.get(child).unwrap();
                    }

                    let ret = folders
                        .get(child)
                        .unwrap()
                        .calculate_size(&folders, size_cache);
                    size_cache.insert(child, ret);
                    ret
                })
                .sum::<usize>()
    }
}

fn main() {
    let _ = input::apply("input-day-07.txt", |input| {
        aoc_lib::timed(|| {
            let mut folders: HashMap<String, Dir> = HashMap::new();
            let mut folder_stack: Vec<String> = Vec::new();
            folders.insert("/".to_string(), Dir::new());
            folder_stack.push("/".to_string());

            for line in input.lines() {
                let mut line = line.split_whitespace();
                let first = line.next().unwrap();
                if first == "$" {
                    if line.next().unwrap() == "cd" {
                        let target = line.next().unwrap();
                        match target {
                            ".." => {
                                folder_stack.pop();
                            }
                            target_name => {
                                let last = folder_stack.last().unwrap();

                                let target_name = last.to_string() + "/" + target_name;
                                if !folders.contains_key(target_name.as_str()) {
                                    folders.insert(target_name.clone(), Dir::new());
                                }

                                if let Some(last) = folders.get_mut(last) {
                                    last.children.push(target_name.clone());
                                }

                                folder_stack.push(target_name);
                            }
                        }
                    }
                } else {
                    match first {
                        "dir" => {}
                        nomber => {
                            let curr_dir = folder_stack.last().unwrap();
                            folders.get_mut(curr_dir).unwrap().size +=
                                nomber.parse::<usize>().unwrap();
                        }
                    }
                }
            }

            aoc_lib::timed(|| println!("{}", part1(&folders)));
            aoc_lib::timed(|| println!("{}", part2(&folders)));
        });
    });
}

fn part1(folders: &HashMap<String, Dir>) -> usize {
    let mut cache = HashMap::new();
    folders
        .values()
        .map(|dir| dir.calculate_size(&folders, &mut cache))
        .filter(|size| *size <= 100_000)
        .sum()
}

const MAX_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn part2(folders: &HashMap<String, Dir>) -> usize {
    let mut cache = HashMap::new();
    let root_size = folders
        .get("/")
        .unwrap()
        .calculate_size(folders, &mut cache);
    let target_size = UPDATE_SIZE - (MAX_SIZE - root_size);

    let mut sorted = folders
        .iter()
        .map(|(_, dir)| dir.calculate_size(folders, &mut cache))
        .collect::<Vec<usize>>();

    sorted.sort();
    *sorted.iter().find(|&&size| size >= target_size).unwrap()
}
