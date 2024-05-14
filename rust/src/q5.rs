/*!
 * A square has two diagonals (lines that run from one corner to a different corner, but not between two corners on the same edge).
 * How many diagonals does a pentagon have?
 * What about a hexagon?
 * What kind of regular polygon has a number of diagonals that is three times the number of its corners?
 */

pub fn solve(n: i32) {
    let diagonals = if n <= 3 { 0 } else { (n - 3) * n / 2 };
    println!("{diagonals}")
}
