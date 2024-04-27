//! 20 April 2024|New Scientist|45
//! arrange the digits 1-9 in a line so that each pair of adjacent digits differs by either 2 or 3

use std::cell::RefCell;

const COLLECTION: [i8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn solve() {
    let mut result: Vec<String> = vec![];

    COLLECTION.iter().for_each(|&elem| {
        Tree::traverse(elem, RefCell::new(vec![]), &mut result);
    });

    println!("there r {} ways to arrange the digits", result.len());
    result.iter().for_each(|s| println!("{s:?}"))
}

struct Tree {
    _root: i8,
    _children: Option<Vec<Tree>>,
}

impl Tree {
    fn traverse(node: i8, visited: RefCell<Vec<i8>>, result: &mut Vec<String>) -> Tree {
        visited.borrow_mut().push(node);
        if visited.borrow().len() == 9 {
            let s = format!("{:?}", visited.borrow());
            result.push(s)
        }

        Tree {
            _root: node,
            _children: get_children_values(node).map(|values| {
                values
                    .into_iter()
                    .filter(|v| !visited.borrow().contains(v))
                    .map(|v| Self::traverse(v, visited.clone(), result))
                    .collect()
            }),
        }
    }
}

fn get_children_values(n: i8) -> Option<Vec<i8>> {
    let mut values: Vec<i8> = vec![];
    if n - 2 >= 1 {
        values.push(n - 2)
    }
    if n - 3 >= 1 {
        values.push(n - 3)
    }
    if n + 2 <= 9 {
        values.push(n + 2)
    }
    if n + 3 <= 9 {
        values.push(n + 3)
    }

    if values.is_empty() {
        return None;
    }

    Some(values)
}
