/*!
 * 4 May 2024|New Scientist|45
 * You must take two steps to get from a given number to make 10. Each step must
 * change the number by adding, subtracting, multiplying by or dividing by a number from 1 to 9.
 * (Multiplying or dividing by 1 isn’t allowed as it doesn’t change the number.)
 */

pub fn solve(n: i32) {
    println!("solve for {n:?}");
    if (-8..=99).contains(&n) {
        println!("integers between (-8..=99) can");
        return;
    }

    let mut list: Vec<String> = vec![];
    for i in 100..=n {
        if let Some(res) = can(i) {
            list.push(res);
        }
    }
    println!("integers between (-8..=99) can, and these numbers can too {list:?}")
}

fn can(n: i32) -> Option<String> {
    if (-8..=99).contains(&n) {
        return Some("integers between (-8..=99) can".to_string());
    }

    for i in (2..=9).rev() {
        let remainder = n % i;
        if remainder == 0 {
            let quotient = n / i;
            if quotient <= 19
                || (20..=90)
                    .step_by(10)
                    .collect::<Vec<i32>>()
                    .contains(&quotient)
            {
                let next = if quotient % 10 == 0 {
                    format!("/{}", quotient / 10)
                } else {
                    match quotient - 10 {
                        x if x > 0 => format!("-{}", quotient - 10),
                        _ => format!("+{}", (quotient - 10).abs()),
                    }
                };
                let result = format!("{n}: {n}/{i} -> {quotient}{next}");
                return Some(result);
            }
        }
    }

    None
}
