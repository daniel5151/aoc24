// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

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

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    for _ in 0..25 {
        for stone in std::mem::take(&mut input) {
            if stone == 0 {
                input.push(1);
            } else {
                let digs = stone.ilog10() + 1;
                if digs % 2 == 0 {
                    let split = 10usize.pow(digs / 2);
                    let top = stone / split;
                    let bottom = stone - (top * split);
                    input.push(top);
                    input.push(bottom);
                } else {
                    input.push(stone * 2024);
                }
            }
        }
    }

    Ok(input.len())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    // ayy lmao can you imagine? but nahh, I know, the solution requires
    // identifying the pattern to efficiently compute this.
    //
    // alas - I've gotta pack for vacation, so this might be the end of the line
    // this year...
    for _ in 0..75 {
        for stone in std::mem::take(&mut input) {
            if stone == 0 {
                input.push(1);
            } else {
                let digs = stone.ilog10() + 1;
                if digs % 2 == 0 {
                    let split = 10usize.pow(digs / 2);
                    let top = stone / split;
                    let bottom = stone - (top * split);
                    input.push(top);
                    input.push(bottom);
                } else {
                    input.push(stone * 2024);
                }
            }
        }
    }

    Ok(input.len())
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

    // #[test]
    // fn q2_e1() {
    //     let input = EXAMPLE_1;
    //     let expected = { 0 };
    //     let q = q2;

    //     assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    // }
}
