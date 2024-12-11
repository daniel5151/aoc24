use crate::prelude::*;

type Answer = usize;

type Input = Vec<usize>;
fn munge_input(input: &str) -> DynResult<Input> {
    let input = input
        .split(' ')
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;
    Ok(input)
}

fn solve(input: &str, iters: usize) -> DynResult<Answer> {
    let input = munge_input(input)?;

    let mut counts = BTreeMap::<usize, usize>::new();
    for init in input {
        *counts.entry(init).or_default() += 1;
    }

    for _ in 0..iters {
        for (stone, count) in std::mem::take(&mut counts) {
            if stone == 0 {
                *counts.entry(1).or_default() += count;
            } else {
                let digs = stone.ilog10() + 1;
                if digs % 2 == 0 {
                    let split = 10usize.pow(digs / 2);
                    let top = stone / split;
                    let bottom = stone - (top * split);
                    *counts.entry(top).or_default() += count;
                    *counts.entry(bottom).or_default() += count;
                } else {
                    *counts.entry(stone * 2024).or_default() += count;
                }
            }
        }
    }

    Ok(counts.values().sum())
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    solve(input, 25)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
125 17
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 55312 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
