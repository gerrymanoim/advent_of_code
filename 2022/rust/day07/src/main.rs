use std::collections::HashMap;

struct File {
    size: usize,
    name: String,
}

struct Directory {
    directories: Vec<Directory>,
    files: Vec<File>,
}

fn execute_command<'a>(
    session: &Vec<&'a str>,
    session_pntr: usize,
    current_dir: &'a str,
    mut parents_to_children: &mut HashMap<&'a str, Vec<&'a str>>,
    mut children_to_parents: &mut HashMap<&'a str, Vec<&'a str>>,
    mut directory_stats: &mut HashMap<&'a str, usize>,
) -> (Option<&'a str>, usize) {
    let cmd = session[session_pntr];
    dbg!(cmd);
    if cmd.starts_with("$ cd ") {
        (
            Some(cmd.strip_prefix("$ cd ").expect("correctly prefixed str")),
            1,
        )
    } else if cmd == "$ ls" {
        let mut file_sizes = 0;
        let mut lines_processed = 0;
        let directory_children = parents_to_children.entry(current_dir).or_default();
        for (n, line) in session[session_pntr+1..].iter().enumerate() {
            dbg!(line);
            if line.starts_with("$") {
                lines_processed = n + 2;
                break;
            } else if line.starts_with("dir") {
                let child_dir = line.strip_prefix("dir ").expect("A directory");
                directory_children.push(child_dir);
                children_to_parents
                    .entry(child_dir)
                    .and_modify(|e| e.push(current_dir))
                    .or_insert(vec![current_dir]);
            } else {
                let (size_str, _) = line.split_once(" ").expect("this should be a file");
                let size: usize = size_str.parse().expect("A file size");
                file_sizes += size;
            }
        }
        for directory in children_to_parents
            .get(current_dir)
            .expect("expect child to exist")
        {
            *directory_stats.entry(directory).or_insert(0) += file_sizes;
        }
        (None, lines_processed)
    } else {
        panic!("Something has gone wrong")
    }
}

// TODO ew this sucks
// fn find_parents<'a>(
//     current_dir: &'a str,
//     directory_tree: &HashMap<&'a str, Vec<&'a str>>,
// ) -> Vec<&'a str> {
//     todo!()
// }

fn build_directory_stats<'a>(session: &'a Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut parents_to_children = HashMap::new();
    let mut children_to_parents = HashMap::new();
    let mut current_dir = "/";
    parents_to_children.insert(current_dir, Vec::new());
    children_to_parents.insert(current_dir, Vec::new());
    let mut directory_stats: HashMap<&str, usize> = HashMap::new();
    let mut session_pntr = 0;
    loop {
        let (maybe_new_dir, steps) = execute_command(
            session,
            session_pntr,
            current_dir,
            &mut parents_to_children,
            &mut children_to_parents,
            &mut directory_stats,
        );
        if maybe_new_dir.is_some() {
            current_dir = maybe_new_dir.unwrap();
        }
        session_pntr += steps;
        dbg!(session_pntr, session.len());
        if session_pntr + 2 == session.len() {
            break;
        }
    }
    directory_stats
}

fn part_1(session: &Vec<&str>) -> usize {
    let directory_stats = build_directory_stats(session);
    directory_stats
        .values()
        .filter(|v| **v <= 100000 as usize)
        .sum()
}

fn main() {
    let session = include_str!("../../../inputs/input07.txt")
        .lines()
        .collect::<Vec<&str>>();
    println!("{}", part_1(&session));
}
