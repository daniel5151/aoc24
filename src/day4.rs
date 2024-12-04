use crate::prelude::*;

type Answer = usize;

type Input = Vec<Vec<u8>>;
fn munge_input(input: &str) -> DynResult<Input> {
    Ok(input.split('\n').map(|x| x.as_bytes().to_vec()).collect())
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;

    let mut found = 0;
    for i in 0..(input.len() as i32) {
        for j in 0..(input[0].len() as i32) {
            if input[i as usize][j as usize] != b'X' {
                continue;
            }

            let check_dirs = [
                [(0, 1), (0, 2), (0, 3)],
                [(0, -1), (0, -2), (0, -3)],
                [(1, 0), (2, 0), (3, 0)],
                [(-1, 0), (-2, 0), (-3, 0)],
                [(1, 1), (2, 2), (3, 3)],
                [(-1, -1), (-2, -2), (-3, -3)],
                [(-1, 1), (-2, 2), (-3, 3)],
                [(1, -1), (2, -2), (3, -3)],
            ];

            'x: for c in check_dirs {
                for (&(x, y), c) in c.iter().zip([b'M', b'A', b'S']) {
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
            }
        }
    }

    Ok(found)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let input = munge_input(input)?;

    let mut found = 0;
    for i in 0..(input.len() as i32) {
        for j in 0..(input[0].len() as i32) {
            if input[i as usize][j as usize] != b'A' {
                continue;
            }

            let opposites = [((-1, -1), (1, 1)), ((1, -1), (-1, 1))];

            let mut lines = 0;
            for ((x1, y1), (x2, y2)) in opposites {
                let c1 = input
                    .get((i + x1) as usize)
                    .cloned()
                    .unwrap_or_default()
                    .get((j + y1) as usize)
                    .cloned()
                    .unwrap_or_default();
                let c2 = input
                    .get((i + x2) as usize)
                    .cloned()
                    .unwrap_or_default()
                    .get((j + y2) as usize)
                    .cloned()
                    .unwrap_or_default();

                if (c1 == b'M' && c2 == b'S') || (c1 == b'S' && c2 == b'M') {
                    lines += 1;
                }
            }

            if lines == 2 {
                found += 1;
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
