// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

type Input = Vec<(usize, Vec<usize>)>;
fn munge_input(input: &str) -> DynResult<Input> {
    input
        .split('\n')
        .map(|s| -> DynResult<_> {
            let (a, list) = s.split_once(": ").ok_or("missing ':'")?;
            Ok((
                a.parse::<usize>()?,
                list.split(' ')
                    .map(|x| x.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let mut total = 0;
    'outer: for (true_ans, nums) in input {
        for ops_bitmap in 0..(1 << (nums.len() - 1)) {
            let mut ops = Vec::new();
            for bit in 0..(nums.len() - 1) {
                if ops_bitmap & (1 << bit) != 0 {
                    ops.push("+");
                } else {
                    ops.push("*");
                }
            }

            let mut ans = if ops[0] == "+" {
                nums[0] + nums[1]
            } else {
                nums[0] * nums[1]
            };
            for (i, x) in nums.iter().enumerate().skip(2) {
                ans = if ops[i - 1] == "+" { ans + x } else { ans * x }
            }

            if ans == true_ans {
                total += ans;
                continue 'outer;
            }
        }
    }

    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let mut total = 0;
    'outer: for (true_ans, nums) in input {
        for mut ops_id in 0..3usize.pow((nums.len() - 1) as u32) {
            let mut ans = nums[0];
            for i in 0..(nums.len() - 1) {
                match ops_id % 3 {
                    0 => ans += nums[i + 1],
                    1 => ans *= nums[i + 1],
                    2 => ans = format!("{}{}", ans, nums[i + 1]).parse().unwrap(),
                    _ => unreachable!(),
                }
                ops_id /= 3;
            }

            if ans == true_ans {
                total += ans;
                continue 'outer;
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 3749 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 11387 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
