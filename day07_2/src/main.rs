use std::collections::BTreeMap;

use aoc::aoc;

const FS_SIZE: u64 = 70_000_000;
const SIZE_NEEDED: u64 = 30_000_000;

#[aoc(2022, 7, 2)]
fn main(input: &str) -> u64 {
    let mut outputs = input.lines().map(Output::parse);
    let mut root = Dir::new();

    reverse_engineer_fs(&mut root, &mut outputs);

    let root_total = walk_dir_sizes(&root, &mut |_| ());
    let free_space = FS_SIZE - root_total;
    let need_to_free = SIZE_NEEDED - free_space;
    let mut to_free = FS_SIZE;

    walk_dir_sizes(&root, &mut |size| if need_to_free <= size && size < to_free {
        to_free = size;
    });

    to_free
}

#[derive(Debug)]
enum Output {
    Cd(String),
    Ls,
    Dir(String),
    File(String, u64),
}

impl Output {
    fn parse(s: &str) -> Self {
        if s == "$ ls" {
            return Output::Ls;
        }

        if s.starts_with("$ cd ") {
            return Output::Cd(s.strip_prefix("$ cd ").unwrap().into());
        }

        if s.starts_with("dir ") {
            return Output::Dir(s.strip_prefix("dir ").unwrap().into());
        }

        let (size, name) = s.split_once(' ').unwrap();
        let size = size.parse::<u64>().unwrap();
        let name = name.to_owned();

        Output::File(name, size)
    }
}

fn reverse_engineer_fs(dir: &mut Dir, outputs: &mut impl Iterator<Item = Output>) {
    while let Some(entry) = outputs.next() {
        match entry {
            Output::Cd(name) => {
                if name == "/" {
                    continue;
                }

                if name == ".." {
                    return;
                }

                reverse_engineer_fs(dir.cd_mut(&name), outputs);
            }
            Output::Ls => continue,
            Output::Dir(name) => dir.mkdir(&name),
            Output::File(name, size) => dir.touch(&name, size),
        }
    }
}

fn walk_dir_sizes(dir: &Dir, f: &mut impl FnMut(u64)) -> u64 {
    let mut total = 0;

    for (_, entry) in dir.entries() {
        match entry {
            Entry::Dir(dir) => total += walk_dir_sizes(dir, f),
            Entry::File(size) => total += size,
        }
    }

    f(total);

    total
}

#[derive(Debug)]
struct Dir {
    entries: BTreeMap<String, Entry>,
}

impl Dir {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn cd_mut(&mut self, name: &str) -> &mut Dir {
        let entry = self
            .entries
            .get_mut(name)
            .unwrap_or_else(|| panic!("dir does not exist: {name:?}"));

        let Entry::Dir(dir) = entry else {
            panic!("not a dir: {name:?}");
        };

        dir
    }

    fn entries(&self) -> impl Iterator<Item = (&str, &Entry)> {
        self.entries.iter().map(|(name, entry)| (&**name, entry))
    }

    fn mkdir(&mut self, name: &str) {
        self.entries.insert(name.into(), Entry::Dir(Dir::new()));
    }

    fn touch(&mut self, name: &str, size: u64) {
        self.entries.insert(name.into(), Entry::File(size));
    }
}

#[derive(Debug)]
enum Entry {
    Dir(Dir),
    File(u64),
}
