use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use aoc_runner_derive::aoc;

enum Cmd<'s> {
    Cd(&'s str),
    Ls,
}

enum Line<'s> {
    Cmd(Cmd<'s>),
    Dir(&'s str),
    File(&'s str, u32),
}

struct Directory<'s> {
    dirs: Vec<&'s str>,
    files: Vec<(&'s str, u32)>,
}

fn filesystem<'s>(input: &'s str) -> HashMap<PathBuf, Directory<'s>> {
    let terminal = input.split('\n').map(|line| {
        if let Some((_, path)) = line.split_once("$ cd ") {
            Line::Cmd(Cmd::Cd(path))
        } else if line.starts_with("$ ls") {
            Line::Cmd(Cmd::Ls)
        } else if let Some((_, dir_name)) = line.split_once("dir ") {
            Line::Dir(dir_name)
        } else {
            let (file_size, file_name) = line.split_once(' ').unwrap();
            let file_size = file_size.parse::<u32>().unwrap();

            Line::File(file_name, file_size)
        }
    });

    let mut filesystem = HashMap::new();
    let mut path = PathBuf::new();

    for line in terminal {
        match line {
            Line::Cmd(Cmd::Cd(dir)) => {
                if dir == ".." {
                    assert!(path.pop());
                    continue;
                }

                path.push(dir);
                assert!(filesystem
                    .insert(
                        path.clone(),
                        Directory {
                            dirs: Vec::new(),
                            files: Vec::new(),
                        }
                    )
                    .is_none());
            }
            Line::Cmd(Cmd::Ls) => {} // No-op
            Line::Dir(name) => {
                filesystem.get_mut(&path).unwrap().dirs.push(name);
            }
            Line::File(name, size) => {
                filesystem.get_mut(&path).unwrap().files.push((name, size));
            }
        }
    }

    filesystem
}

fn recurse<F: FnMut(u32)>(fs: &HashMap<PathBuf, Directory<'_>>, path: &Path, f: &mut F) -> u32 {
    let dir = &fs[path];
    let mut size = 0;

    for dir_name in &dir.dirs {
        let path = path.join(dir_name);
        let sub_size = recurse(fs, &path, f);

        size += sub_size;
    }

    for (_file_name, sub_size) in &dir.files {
        size += sub_size;
    }

    f(size);

    size
}

#[aoc(day7, part1, Chars)]
pub fn part1_chars(input: &str) -> u32 {
    let fs = filesystem(input);
    let mut total = 0;
    recurse(&fs, Path::new("/"), &mut |size| {
        if size <= 100_000 {
            total += size;
        }
    });
    total
}

#[aoc(day7, part2, Chars)]
pub fn part2_chars(input: &str) -> u32 {
    let fs = filesystem(input);
    let mut perfect_size = u32::MAX;
    let total_size = recurse(&fs, Path::new("/"), &mut |_| {});
    let free_space = 70000000 - total_size;
    recurse(&fs, Path::new("/"), &mut |size| {
        if size + free_space >= 30000000 && size < perfect_size {
            perfect_size = size;
        }
    });

    perfect_size
}
