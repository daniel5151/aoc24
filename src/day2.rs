// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

type Input = Vec<Vec<isize>>;
fn munge_input(input: &str) -> DynResult<Input> {
    let mut input = input.split('\n');

    // let init = input.next().unwrap();

    let input = input
        .map(|s| -> DynResult<_> {
            let res = {
                // parse
                s.split(' ')
                    .map(|x| x.parse::<isize>())
                    .collect::<Result<Vec<_>, _>>()?
            };
            Ok(res)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(input)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;

    let mut safe = 0;
    'outer: for list in input {
        let is_inc = list[1] > list[0];

        for &[a, b] in list.array_windows() {
            let diff = if is_inc { b - a } else { a - b };
            if diff > 3 || diff <= 0 {
                continue 'outer;
            }
        }
        safe += 1;
    }

    Ok(safe)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;

    let mut safe = 0;
    for list in input {
        // YOLO brute force solution
        'outer: for i in 0..list.len() {
            let mut list = list.clone();
            list.remove(i);

            let is_inc = list[1] > list[0];
            for &[a, b] in list.array_windows() {
                let diff = if is_inc { b - a } else { a - b };
                if diff > 3 || diff <= 0 {
                    continue 'outer;
                }
            }

            safe += 1;
            break;
        }
    }

    Ok(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 2 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 4 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
