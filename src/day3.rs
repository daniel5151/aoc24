use crate::prelude::*;

type Answer = usize;

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
    MulP(usize),
    MulPC(usize, usize),
}

fn exec(input: &str, always_enabled: bool) -> usize {
    let mut total = 0;
    let mut enabled = true;
    let mut candidate_enable = false;
    let mut state = State::None;
    for c in input.chars() {
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
                if !always_enabled {
                    enabled = candidate_enable;
                }
                State::None
            }

            ('m', State::None) => State::M,
            ('u', State::M) => State::Mu,
            ('l', State::Mu) => State::Mul,
            ('(', State::Mul) => State::MulP(0),
            ('0'..='9', State::MulP(prev)) => State::MulP(prev * 10 + (c as u8 - b'0') as usize),
            (',', State::MulP(prev)) => State::MulPC(prev, 0),
            ('0'..='9', State::MulPC(first, prev)) => {
                State::MulPC(first, prev * 10 + (c as u8 - b'0') as usize)
            }
            (')', State::MulPC(a, b)) => {
                if enabled {
                    total += a * b;
                }
                State::None
            }
            _ => State::None,
        }
    }

    total
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    Ok(exec(input, true))
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    Ok(exec(input, false))
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
