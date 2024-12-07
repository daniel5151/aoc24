// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

type Input = Vec<usize>;
fn munge_input(input: &str) -> DynResult<Input> {
    let mut input = input.split('\n');

    // let init = input.next().unwrap();

    let input = input
        .map(|s| -> DynResult<_> {
            let res = {
                // parse
                // s
                todo!()
            };
            Ok(res)
        })
        .collect::<Result<_, _>>()?;

    Ok(input)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let _ = input;

    todo!()
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let _ = input;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
.....dummy.....
.....input.....
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 0 };
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
