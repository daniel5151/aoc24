// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

// type Input = Vec<usize>;
// fn munge_input(input: &str) -> DynResult<Input> {}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split('\n');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = {
                    // parse
                    s.to_string()
                };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut total = 0;
    for line in input {
        #[derive(Copy, Clone)]
        enum State {
            None,
            M,
            Mu,
            Mul,
            MulB(usize),
            MulBC(usize, usize),
        }

        let mut state = State::None;
        for c in line.chars() {
            state = match (c, state) {
                ('m', State::None) => State::M,
                ('u', State::M) => State::Mu,
                ('l', State::Mu) => State::Mul,
                ('(', State::Mul) => State::MulB(0),
                ('0'..='9', State::MulB(prev)) => {
                    State::MulB(prev * 10 + (c as u8 - b'0') as usize)
                }
                (',', State::MulB(prev)) => State::MulBC(prev, 0),
                ('0'..='9', State::MulBC(first, prev)) => {
                    State::MulBC(first, prev * 10 + (c as u8 - b'0') as usize)
                }
                (')', State::MulBC(a, b)) => {
                    total += a * b;
                    State::None
                }
                _ => State::None,
            }
        }
    }
    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split('\n');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = {
                    // parse
                    s.to_string()
                };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut enabled = true;

    let mut total = 0;
    for line in input {
        #[derive(Copy, Clone)]
        enum State {
            None,

            D,
            Do,
            Don,
            DonA,
            DonAt,
            CloseParen,

            M,
            Mu,
            Mul,
            MulB(usize),
            MulBC(usize, usize),
        }

        let mut candidate_enable = false;

        let mut state = State::None;
        for c in line.chars() {
            state = match (c, state) {
                ('d', State::None) => State::D,
                ('o', State::D) => State::Do,

                ('(', State::Do) => {
                    candidate_enable = true;
                    State::CloseParen
                }

                ('n', State::Do) => State::Don,
                ('\'', State::Don) => State::DonA,
                ('t', State::DonA) => State::DonAt,
                ('(', State::DonAt) => {
                    candidate_enable = false;
                    State::CloseParen
                }

                (')', State::CloseParen) => {
                    enabled = candidate_enable;
                    State::None
                }

                ('m', State::None) => State::M,
                ('u', State::M) => State::Mu,
                ('l', State::Mu) => State::Mul,
                ('(', State::Mul) => State::MulB(0),
                ('0'..='9', State::MulB(prev)) => {
                    State::MulB(prev * 10 + (c as u8 - b'0') as usize)
                }
                (',', State::MulB(prev)) => State::MulBC(prev, 0),
                ('0'..='9', State::MulBC(first, prev)) => {
                    State::MulBC(first, prev * 10 + (c as u8 - b'0') as usize)
                }
                (')', State::MulBC(a, b)) => {
                    if enabled {
                        total += a * b;
                    }
                    State::None
                }
                _ => State::None,
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 161 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    const EXAMPLE_2: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_2;
        let expected = { 48 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
