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

// this fits my input ¯\_(ツ)_/¯
fn concat(a: usize, b: usize) -> usize {
    match a {
        _ if b < 10 => a * 10 + b,
        _ if b < 100 => a * 100 + b,
        _ if b < 1000 => a * 1000 + b,
        _ => unreachable!(),
    }
}

fn solve<const BASE: usize>(input: Input) -> Answer {
    let mut total = 0;
    'outer: for (true_ans, nums) in input {
        for mut ops_id in 0..BASE.pow((nums.len() - 1) as u32) {
            let mut ans = nums[0];
            for &n in nums.iter().skip(1) {
                match ops_id % BASE {
                    0 => ans += n,
                    1 => ans *= n,
                    2 => ans = concat(ans, n),
                    _ => unreachable!(),
                }
                ops_id /= BASE;
            }

            if ans == true_ans {
                total += ans;
                continue 'outer;
            }
        }
    }
    total
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;
    Ok(solve::<2>(input))
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;
    Ok(solve::<3>(input))
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
