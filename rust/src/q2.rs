/*!
 * 13 Jan 2024|New Scientist|45
 * You are given a heap of N tokens, which you may divide into any number of smaller heaps. You will then receive an
 * amount of money equal to the product of the number of tokens in each heap. The rules of this game state that if you
 * were to just leave all the tokens in one heap, you would win £N.
 *
 * What is the largest sum of money you can win starting with six tokens?
 * What about starting with 10 tokens?
 * Is there a general best strategy for N tokens?
 */

pub fn solve(n: i64) {
    let mut product = n;
    let mut bucket: i64;

    for rest in 0..n {
        bucket = n - rest;
        let p = match rest {
            _ if rest == bucket => 2_i64.pow(rest.try_into().unwrap()),
            _ if bucket == 1 => n,
            _ if rest > bucket => {
                let quotient = rest / bucket;
                let remainder = rest % bucket;
                let factor1 = (quotient + 1).pow((bucket - remainder).try_into().unwrap());
                let factor2 = (quotient + 1 + 1).pow(remainder.try_into().unwrap());
                // println!(
                //     "{rest}-{bucket} -- {quotient}-{remainder} -- {factor1} * {factor2} = {:?}",
                //     factor1 * factor2
                // );
                factor1 * factor2
            }
            _ => 0,
        };

        if p > product {
            product = p;
        }
    }

    println!("{product}")
}
