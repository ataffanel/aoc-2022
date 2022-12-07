use anyhow::Error;
use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
    str::FromStr,
    vec,
};

#[derive(Debug)]
enum Line {
    Cd(String),
    Ls,
    File { size: usize, name: String },
    Dir { name: String },
}

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<Rc<RefCell<File>>>,
    parent: Option<Weak<RefCell<Dir>>>,
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
    parent: Option<Weak<RefCell<Dir>>>,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd") {
            let name = s.split(" ").nth(2).unwrap().into();
            Ok(Line::Cd(name))
        } else if s.starts_with("$ ls") {
            Ok(Line::Ls)
        } else if s.starts_with("dir") {
            let name = s.split(" ").nth(1).unwrap().into();
            Ok(Self::Dir { name })
        } else {
            let mut elements = s.split(" ");
            let size: usize = elements.next().unwrap().parse()?;
            let name = elements.next().unwrap().into();
            Ok(Self::File { size, name })
        }
    }
}

impl TryInto<Dir> for Line {
    type Error = Error;

    fn try_into(self) -> Result<Dir, Self::Error> {
        match self {
            Self::Dir { name } => Ok(Dir {
                name,
                subdirs: vec![],
                files: vec![],
                parent: None,
            }),
            _ => Err(Error::msg("Not a directory")),
        }
    }
}

impl TryInto<File> for Line {
    type Error = Error;

    fn try_into(self) -> Result<File, Self::Error> {
        match self {
            Self::File { name, size } => Ok(File {
                _name: name,
                size,
                parent: None,
            }),
            _ => Err(Error::msg("Not a file")),
        }
    }
}

impl Dir {
    fn size(&self) -> usize {
        let subdir_size: usize = self
            .subdirs
            .iter()
            .map(|dir| RefCell::<Dir>::borrow(&dir).size())
            .sum();
        let file_size: usize = self
            .files
            .iter()
            .map(|file| RefCell::<File>::borrow(&file).size)
            .sum();

        subdir_size + file_size
    }

    fn all_dirs(&self) -> Vec<Rc<RefCell<Dir>>> {
        let mut dirs: Vec<Rc<RefCell<Dir>>> = self.subdirs.iter().map(|r| r.clone()).collect();

        for subdir in self.borrow().subdirs.iter() {
            for dir in RefCell::<Dir>::borrow(&subdir).all_dirs().iter() {
                dirs.push(dir.clone());
            }
        }

        dirs
    }
}

const MAX_SIZE: usize = 100000;
const AVAILABLE_SPACE: usize = 70000000;
const UNUSED_SPACE_REQUIRED: usize = 30000000;
const MAX_USED_SPACE: usize = AVAILABLE_SPACE - UNUSED_SPACE_REQUIRED;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?;
    let lines = input.lines().map(|line| Line::from_str(line).unwrap());

    let root: Rc<RefCell<Dir>> = Rc::new(RefCell::new(Dir {
        name: "/".into(),
        files: vec![],
        subdirs: vec![],
        parent: None,
    }));

    let mut current_dir = root.clone();

    for line in lines {
        match line {
            Line::Ls => (),
            Line::Dir { .. } => {
                let dir = Rc::new(RefCell::<Dir>::new(line.try_into()?));
                (*dir.borrow_mut()).parent = Some(Rc::downgrade(&current_dir));
                current_dir.borrow_mut().subdirs.push(dir);
            }
            Line::File { .. } => {
                let file = Rc::new(RefCell::<File>::new(line.try_into()?));
                (*file.borrow_mut()).parent = Some(Rc::downgrade(&current_dir));
                current_dir.borrow_mut().files.push(file);
            }
            Line::Cd(name) if name == "/" => {
                current_dir = root.clone();
            }
            Line::Cd(name) if name == ".." => {
                let parent = RefCell::<Dir>::borrow(&current_dir)
                    .parent
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                current_dir = parent;
            }
            Line::Cd(name) => {
                let new_dir = RefCell::<Dir>::borrow(&current_dir)
                    .subdirs
                    .iter()
                    .filter(|subdir| RefCell::<Dir>::borrow(&subdir).name == name)
                    .next()
                    .unwrap()
                    .clone();
                current_dir = new_dir;
            }
        }
    }

    let sum: usize = RefCell::<Dir>::borrow(&root)
        .all_dirs()
        .iter()
        .map(|dir| RefCell::<Dir>::borrow(&dir).size())
        .filter(|size| *size <= MAX_SIZE)
        .sum();

    println!("Sum of the small folders: {}", sum);

    let currently_used_space = RefCell::<Dir>::borrow(&root).size();
    let to_free = currently_used_space - MAX_USED_SPACE;

    let mut candidate_sizes: Vec<_> = RefCell::<Dir>::borrow(&root)
        .all_dirs()
        .iter()
        .map(|dir| RefCell::<Dir>::borrow(&dir).size())
        .filter(|size| *size >= to_free)
        .collect();

    candidate_sizes.sort();

    println!(
        "Size of the smallest folder to delete: {}",
        candidate_sizes.get(0).unwrap()
    );

    Ok(())
}
