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
                let res = { s.chars().collect::<Vec<char>>() };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut found = 0;
    for i in 0..(input.len() as i32) {
        for j in 0..(input[0].len() as i32) {
            let check_dirs = [
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 0), (0, -1), (0, -2), (0, -3)],
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
                [(0, 0), (1, 1), (2, 2), (3, 3)],
                [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
                [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
                [(0, 0), (1, -1), (2, -2), (3, -3)],
            ];

            'x: for c in check_dirs {
                for (&(x, y), c) in c.iter().zip(['X', 'M', 'A', 'S']) {
                    if input
                        .get((i + x) as usize)
                        .cloned()
                        .unwrap_or_default()
                        .get((j + y) as usize)
                        .cloned()
                        .unwrap_or_default()
                        != c
                    {
                        continue 'x;
                    }
                }
                found += 1;
                // break;
            }
        }
    }

    Ok(found)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = {
        let mut input = input.split('\n');

        // let init = input.next().unwrap();

        let input = input
            .map(|s| -> DynResult<_> {
                let res = { s.chars().collect::<Vec<char>>() };
                Ok(res)
            })
            .collect::<Result<Vec<_>, _>>()?;

        input
    };

    let mut found = 0;
    for i in 0..(input.len() as i32) {
        for j in 0..(input[0].len() as i32) {
            if input[i as usize][j as usize] != 'A' {
                continue;
            }

            let orients = [
                [((-1, -1), (1, 1)), ((1, -1), (-1, 1))],
                [((-1, 0), (1, 0)), ((0, 1), (0, -1))],
            ];

            for orient in orients {
                let mut bad = false;
                for (c1, c2) in orient {
                    let c1 = input
                        .get((i + c1.0) as usize)
                        .cloned()
                        .unwrap_or_default()
                        .get((j + c1.1) as usize)
                        .cloned()
                        .unwrap_or_default();
                    let c2 = input
                        .get((i + c2.0) as usize)
                        .cloned()
                        .unwrap_or_default()
                        .get((j + c2.1) as usize)
                        .cloned()
                        .unwrap_or_default();

                    if (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M') {
                        continue;
                    }

                    bad = true;
                }

                if !bad {
                    found += 1;
                }
            }
        }
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 18 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 9 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
