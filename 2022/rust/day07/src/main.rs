use std::collections::HashMap;

fn build_directory_stats(input: &Vec<&str>) -> HashMap<String, usize> {
    let mut session = input.clone();
    session.reverse();

    let mut current_dir = session.pop().unwrap().strip_prefix("$ cd ").unwrap();
    let mut dir_queue = vec![current_dir];

    let mut directory_stats: HashMap<String, usize> = HashMap::new();
    while session.len() > 0 {
        let cmd = session.pop().expect("commands to not be empty");
        if cmd.starts_with("$ cd ") {
            match cmd.strip_prefix("$ cd ") {
                Some("..") => {
                    current_dir = dir_queue.pop().expect("Something above current dir");
                }
                Some(x) => {
                    dir_queue.push(current_dir);
                    current_dir = x;
                }
                None => unreachable!("Not correctly prefixed dir"),
            };
        } else if cmd == "$ ls" {
            let mut file_sizes = 0;
            while (session.len() > 0) && !session.last().unwrap().starts_with("$") {
                let output_line = session.pop().expect("expect a line");
                if output_line.starts_with("dir") {
                    //
                } else {
                    let (size_str, _) = output_line.split_once(" ").expect("this should be a file");
                    let size: usize = size_str.parse().expect("A file size");
                    file_sizes += size;
                }
            }
            // ew
            for n in 0..dir_queue.len() {
                let key = dir_queue[..=n].join("/");
                *directory_stats.entry(key).or_insert(0) += file_sizes;
            }
            let key = dir_queue.join("/") + "/" + current_dir;
            *directory_stats.entry(key).or_insert(0) += file_sizes;
        } else {
            panic!("Something has gone wrong")
        }
    }

    directory_stats
}

fn part_1(input: &Vec<&str>) -> usize {
    let directory_stats = build_directory_stats(input);
    directory_stats
        .values()
        .filter(|v| **v <= 100000 as usize)
        .sum()
}

fn part_2(input: &Vec<&str>) -> usize {
    let directory_stats = build_directory_stats(input);
    let total_space = 70000000;
    let unused_needed = 30000000;
    let unused_sapce = total_space-  directory_stats.get("/").unwrap();
    let mut dir_sizes: Vec<&usize> = directory_stats.values().collect();
    dir_sizes.sort();
    for dir_size in dir_sizes {
        if unused_sapce + dir_size > unused_needed {
            return *dir_size
        }
    }
    unreachable!("We should not be here")
}

fn main() {
    let input = include_str!("../../../inputs/input07.txt")
        .lines()
        .collect::<Vec<&str>>();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}


#[test]
fn example() {
    let session = "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k"
        .lines()
        .map(|s| s.trim())
        .collect();
    assert_eq!(part_1(&session), 95437);
}
