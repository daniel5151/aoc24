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
                s.as_bytes().iter().map(|x| x - b'0').collect()
            };
            Ok(res)
        })
        .collect::<Result<_, _>>()?;

    Ok(input)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    // find roots
    let mut roots = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                roots.push((x, y));
            }
        }
    }

    let mut total = 0;
    for root in roots {
        let mut nines: BTreeSet<(usize, usize)> = BTreeSet::new();
        fn find_nines(
            (rx, ry): (usize, usize),
            input: &Input,
            nines: &mut BTreeSet<(usize, usize)>,
        ) {
            let val = input[ry][rx];

            if val == 9 {
                nines.insert((rx, ry));
                return;
            }

            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if (dx, dy) == (0, 0) {
                    continue;
                }

                let (nx, ny) = ((rx as isize + dx) as usize, (ry as isize + dy) as usize);
                let adj = input
                    .get(ny)
                    .cloned()
                    .unwrap_or_default()
                    .get(nx)
                    .cloned()
                    .unwrap_or_default();

                if adj == val + 1 {
                    find_nines((nx, ny), input, nines);
                }
            }
        }
        find_nines(root, &input, &mut nines);
        dbg!(&nines.len());

        total += nines.len();
    }
    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    // find roots
    let mut roots = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                roots.push((x, y));
            }
        }
    }

    let mut total = 0;
    for root in roots {
        let mut nines: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        fn find_nines(
            (rx, ry): (usize, usize),
            input: &Input,
            nines: &mut BTreeMap<(usize, usize), usize>,
        ) {
            let val = input[ry][rx];

            if val == 9 {
                *nines.entry((rx, ry)).or_default() += 1;
                return;
            }

            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if (dx, dy) == (0, 0) {
                    continue;
                }

                let (nx, ny) = ((rx as isize + dx) as usize, (ry as isize + dy) as usize);
                let adj = input
                    .get(ny)
                    .cloned()
                    .unwrap_or_default()
                    .get(nx)
                    .cloned()
                    .unwrap_or_default();

                if adj == val + 1 {
                    find_nines((nx, ny), input, nines);
                }
            }
        }
        find_nines(root, &input, &mut nines);
        dbg!(&nines.len());

        for uniq in nines.values() {
            total += *uniq;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 36 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 81 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
