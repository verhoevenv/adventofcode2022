use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::Lines;

use regex::Regex;

#[derive(Debug)]
pub struct Directory {
    files: Vec<File>,
    dirs: HashMap<String, Directory>
}

impl Directory {
    pub fn empty() -> Self {
        return Directory { files: vec![], dirs: HashMap::new() }
    }

    pub fn from_commands(input: &mut Lines) -> Self {
        let mut dirs = HashMap::new();
        let mut files = Vec::new();
        let cd_in = Regex::new(r"^\$ cd ([a-z]+)$").unwrap();
        let cd_out = Regex::new(r"^\$ cd \.\.$").unwrap();
        let ls_re = Regex::new(r"^\$ ls$").unwrap();
        let dir_re = Regex::new(r"^dir ([a-z]+)$").unwrap();
        let file_re = Regex::new(r"^(\d+) ([a-z.]+)$").unwrap();

        while let Some(line) = input.next() {
            if ls_re.is_match(line) {
            } else if let Some(new_dir) = dir_re.captures(line) {
                dirs.insert(new_dir[1].to_owned(), Directory::empty());
            } else if let Some(new_file) = file_re.captures(line) {
                files.push(File {
                    size: new_file[1].parse().unwrap(),
                });
            } else if let Some(sub_dir) = cd_in.captures(line) {
                let created_dir = Directory::from_commands(input);
                *dirs.get_mut(&sub_dir[1]).unwrap() = created_dir;
            } else if cd_out.is_match(line) {
                break;
            } else {
                panic!("Unknown line {}", line);
            }
            
        }
        return Directory { 
            files,
            dirs,
        };
    }

    pub fn sum_of_small_dirs(&self) -> i32 {
        return self.dirs().iter()
            .filter(|d| d.total_size() < 100000)
            .map(|d| d.total_size())
            .sum();
    }
    
    pub fn dir_with_enough_space(&self) -> i32 {
        let unused_size = 70000000 - self.total_size();
        let needed_size = 30000000 - unused_size;
        let mut okay_dirs: Vec<&Directory> = self.dirs().into_iter()
            .filter(|d| d.total_size() > needed_size)
            .collect();
        okay_dirs.sort_by(|a, b| a.total_size().cmp(&b.total_size()));
        return okay_dirs.get(0).unwrap().total_size();
    }

    pub fn dirs(&self) -> Vec<&Directory> {
        let mut result = Vec::new();
        for (_, d) in &self.dirs {
            result.push(d);
            result.append(&mut d.dirs());
        }
        return result;
    }

    pub fn total_size(&self) -> i32 {
        let subdirs: i32 = self.dirs.iter().map(|(_, d)| d.total_size()).sum();
        let files: i32 = self.files.iter().map(|f| f.size).sum();
        return subdirs + files;
    }
}

#[derive(Debug)]
pub struct File {
    size: i32,
}

pub fn parse(input: &str) -> Directory {
    let mut commands = input.lines();
    // ignore the first cd / line
    commands.next();
    return Directory::from_commands(&mut commands);
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let fs = parse(&input);
    println!("{}", fs.dir_with_enough_space());
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        $ cd /
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
        7214296 k
    "};

    #[test]
    fn test_fs() {
        let fs = parse(INPUT);
        assert_eq!(fs.total_size(), 48381165);
        assert_eq!(fs.sum_of_small_dirs(), 95437);
        assert_eq!(fs.dir_with_enough_space(), 24933642);
    }

}