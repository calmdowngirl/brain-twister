//! extend [q16](https://github.com/calmdowngirl/brain-twister/blob/main/rust/src/q16.rs) to print dir tree
//! usage:
//! ```bash
//! # show current dir
//! ./tree .
//! # show current dir two levels deep, show hidden file
//! ./tree . -d 2 -h true
//! ```

use std::collections::HashMap;
use std::env::{args, current_dir};
use std::fs;
use std::path::Path;

pub fn main() {
    let args: Vec<String> = args().collect();
    let root = if args.len() > 1 {
        Path::new(&args[1]).display().to_string()
    } else {
        current_dir().unwrap().display().to_string()
    };

    let mut max_depth: i32 = 0;
    let mut should_show_hidden = false;
    if args.len() > 3 {
        args.iter().enumerate().for_each(|(i, v)| {
            if ["--depth".to_string(), "-d".to_string()].contains(v) {
                max_depth = args
                    .get(i + 1)
                    .unwrap_or(&0.to_string())
                    .parse()
                    .unwrap_or(0)
            }

            if ["--show-hidden".to_string(), "-h".to_string()].contains(v) {
                should_show_hidden = args
                    .get(i + 1)
                    .unwrap_or(&"false".to_string())
                    .parse()
                    .unwrap_or(false)
            }
        })
    };

    let mut should_draw_bar: Vec<bool> = vec![true];

    Tree::traverse(
        root,
        "".to_string(),
        &mut should_draw_bar,
        &mut HashMap::<i32, Vec<String>>::new(),
        0,
        max_depth,
        should_show_hidden,
    );
}

struct Tree {
    _root: String,
    _children: Option<Vec<Tree>>,
}

impl Tree {
    fn traverse(
        node: String,
        symbol: String,
        should_draw_bar: &mut Vec<bool>,
        visited: &mut HashMap<i32, Vec<String>>,
        mut curr_depth: i32,
        max_depth: i32,
        should_show_hidden: bool,
    ) -> Tree {
        if !is_valid_path(&node) {
            panic!("{} is not a valid path", node)
        }

        if curr_depth == 0 && !is_directory(&node) {
            panic!("{} is not a directory", node)
        }

        if let Some(v) = visited.get_mut(&curr_depth) {
            v.push(node.clone())
        } else {
            visited.insert(curr_depth, vec![node.clone()]);
        }

        let dir_name = get_name(&node);
        let dir_entries_names =
            get_dir_entry_names(node.clone(), curr_depth, max_depth, should_show_hidden);
        let num_names = match &dir_entries_names {
            Some(vs) => vs.len(),
            _ => 0,
        };

        let s = if curr_depth == 0 {
            node.clone()
        } else if curr_depth == 1 {
            format!("{}{}", symbol, dir_name)
        } else {
            let mut pre = String::new();
            for i in 0..curr_depth - 1 {
                if should_draw_bar[i as usize] {
                    pre.push_str("│   ")
                } else {
                    pre.push_str("    ")
                }
            }
            format!("{}{}{}", pre, symbol, dir_name)
        };
        println!("{}", s);
        curr_depth += 1;

        let vc = visited.clone();

        Tree {
            _root: node.clone(),
            _children: dir_entries_names.map(|values| {
                values
                    .into_iter()
                    .enumerate()
                    .filter(|(_, v)| {
                        let value = vc.get(&curr_depth);
                        if value.is_none() || !value.unwrap().contains(v) {
                            return true;
                        }
                        false
                    })
                    .map(|(i, v)| {
                        // follow the previous value
                        let next_bar = *should_draw_bar.last().unwrap();
                        should_draw_bar.push(next_bar);

                        let symbol: String = if i == num_names - 1 {
                            // last sibling, set should_draw_bar value to be false for next iteration
                            should_draw_bar[curr_depth as usize - 1] = false;
                            "└── ".to_string()
                        } else {
                            should_draw_bar[curr_depth as usize - 1] = true;
                            "├── ".to_string()
                        };

                        Self::traverse(
                            v,
                            symbol,
                            should_draw_bar,
                            visited,
                            curr_depth,
                            max_depth,
                            should_show_hidden,
                        )
                    })
                    .collect()
            }),
        }
    }
}

fn get_dir_entry_names(
    path: String,
    curr_depth: i32,
    max_depth: i32,
    should_show_hidden: bool,
) -> Option<Vec<String>> {
    if max_depth > 0 && curr_depth >= max_depth {
        return None;
    }

    // skip symlink
    if path.contains(" -> ") {
        return None;
    }

    let mut sub_dirs: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    let mut names: Vec<String> = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let s = entry.path().display().to_string();
            if get_name(&s).starts_with(".") && !should_show_hidden {
                continue;
            }
            if let Ok(metadata) = fs::symlink_metadata(&s) {
                let file_type = metadata.file_type();
                match file_type {
                    t if t.is_dir() => sub_dirs.push(s),
                    t if t.is_file() => files.push(s),
                    t if t.is_symlink() => {
                        if let Some(target) = entry.path().read_link().ok() {
                            files.push(format!("{} -> {}", get_name(&s), target.display()))
                        }
                    }
                    _ => {}
                }
            }
        }
        sub_dirs.sort();
        files.sort();
        names.extend(files);
        names.extend(sub_dirs);
        return Some(names);
    }

    None
}

fn is_valid_path(p: &str) -> bool {
    // symlink
    if p.contains(" -> ") {
        return true;
    }
    let path = Path::new(&p);
    path.exists()
}

fn is_directory(path: &str) -> bool {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}

fn get_name(p: &str) -> String {
    let parts: Vec<&str> = p.split('/').collect();
    let name = parts.last().unwrap().to_string();
    if name.is_empty() {
        return p.to_string();
    }
    name
}
