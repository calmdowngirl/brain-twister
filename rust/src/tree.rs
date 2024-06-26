//! extend [q16](https://github.com/calmdowngirl/brain-twister/blob/main/rust/src/q16.rs) to print dir tree
//! usage:
//! ```bash
//! # show current dir
//! ./tree .
//! # show current dir two levels deep, show hidden file
//! ./tree . -d 2 -h true
//! ```

use std::env::{args, current_dir};
use std::path::Path;
use std::{fs, io, process};

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
        mut curr_depth: i32,
        max_depth: i32,
        should_show_hidden: bool,
    ) -> Tree {
        let mut dir_name = get_name(&node);

        if curr_depth == 0 && !is_directory(&node) {
            eprintln!("{} is not a valid directory", node);
            process::exit(1);
        }

        if !is_valid_path(&node) {
            panic!("invalid path, this should not happen")
        }

        let mut dir_entries_names = None;
        if is_directory(&node) {
            dir_entries_names =
                match get_dir_entry_names(node.clone(), curr_depth, max_depth, should_show_hidden) {
                    Ok(entries) => entries,
                    Err(e) => {
                        dir_name.push_str(&format!("  [error opening dir {:?}]", e.to_string()));
                        None
                    }
                }
        }

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

        Tree {
            _root: node.clone(),
            _children: dir_entries_names.map(|values| {
                values
                    .into_iter()
                    .enumerate()
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
) -> Result<Option<Vec<String>>, io::Error> {
    if max_depth > 0 && curr_depth >= max_depth {
        return Ok(None);
    }

    // skip symlink
    if path.contains(" -> ") {
        return Ok(None);
    }

    let mut sub_dirs: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    let mut names: Vec<String> = vec![];

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let s = entry.path().display().to_string();
                if get_name(&s).starts_with('.') && !should_show_hidden {
                    continue;
                }
                if let Ok(metadata) = fs::symlink_metadata(&s) {
                    let file_type = metadata.file_type();
                    match file_type {
                        t if t.is_dir() => sub_dirs.push(s),
                        t if t.is_file() => files.push(s),
                        t if t.is_symlink() => {
                            if let Ok(target) = entry.path().read_link() {
                                files.push(format!("{} -> {}", get_name(&s), target.display()))
                            }
                        }
                        _ => files.push(s),
                    }
                }
            }
            sub_dirs.sort();
            files.sort();
            names.extend(files);
            names.extend(sub_dirs);

            if names.is_empty() {
                Ok(None)
            } else {
                Ok(Some(names))
            }
        }
        Err(e) => Err(e),
    }
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
    if let Ok(metadata) = fs::metadata(path) {
        metadata.file_type().is_dir()
    } else {
        false
    }
}

fn get_name(p: &str) -> String {
    // symlink
    if p.contains(" -> ") {
        return p.to_string();
    }
    let parts: Vec<&str> = p.split('/').collect();
    let name = parts.last().unwrap().to_string();
    if name.is_empty() {
        return p.to_string();
    }
    name
}
