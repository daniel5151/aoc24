// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

type Input = Vec<Vec<u8>>;
fn munge_input(input: &str) -> DynResult<Input> {
    let mut input = input.split('\n');

    let input = input
        .map(|s| -> DynResult<_> {
            let res = {
                // parse
                s.as_bytes().to_vec()
            };
            Ok(res)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(input)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let mut pos = (0, 0);
    'outer: for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == b'^' {
                pos = (x as i32, y as i32);
                break 'outer;
            }
        }
    }

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir = 0;

    let mut marked = 1;
    loop {
        let (x, y) = pos;
        let (dx, dy) = dirs[dir];

        let (nx, ny) = ((x + dx) as usize, (y + dy) as usize);

        let Some(row) = input.get(ny) else {
            break;
        };
        let Some(&next) = row.get(nx) else {
            break;
        };

        match next {
            b'.' | b'X' => {
                input[ny][nx] = b'^';
                input[y as usize][x as usize] = b'X';
                pos = (nx as i32, ny as i32);
                if next == b'.' {
                    marked += 1;
                }
            }
            b'#' => {
                // pos unchanged, but dir is diff
                dir = (dir + 1) % 4;
            }
            _ => unreachable!(),
        }
    }

    Ok(marked)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let mut loops = 0;
    for oy in 0..input.len() {
        'brute: for ox in 0..input[0].len() {
            let mut input = input.clone();

            if input[oy][ox] != b'.' {
                continue;
            }

            dbg!(loops);
            dbg!((oy, ox));

            input[oy][ox] = b'#';

            let mut pos = (0, 0);
            'outer: for y in 0..input.len() {
                for x in 0..input[0].len() {
                    if input[y][x] == b'^' {
                        pos = (x as i32, y as i32);
                        break 'outer;
                    }
                }
            }

            let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            let mut dir = 0;

            let mut visit: BTreeSet<((i32, i32), usize)> = BTreeSet::new();

            loop {
                let (x, y) = pos;
                let (dx, dy) = dirs[dir];

                let curr = ((x, y), dir);
                if visit.contains(&curr) {
                    loops += 1;
                    continue 'brute;
                }

                visit.insert(((x, y), dir));

                let (nx, ny) = ((x + dx) as usize, (y + dy) as usize);

                let Some(row) = input.get(ny) else {
                    break;
                };
                let Some(&next) = row.get(nx) else {
                    break;
                };

                match next {
                    b'.' | b'X' => {
                        input[ny][nx] = b'^';
                        input[y as usize][x as usize] = b'X';
                        pos = (nx as i32, ny as i32);
                    }
                    b'#' => {
                        // pos unchanged, but dir is diff
                        dir = (dir + 1) % 4;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    Ok(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 41 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 6 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
