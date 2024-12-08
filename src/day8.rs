// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

type Input = Vec<Vec<u8>>;
fn munge_input(input: &str) -> DynResult<Input> {
    let input = input.split('\n').map(|s| s.as_bytes().to_vec()).collect();
    Ok(input)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let bound_y = input.len() as isize;
    let bound_x = input[0].len() as isize;

    let mut antennas: BTreeMap<u8, Vec<(usize, usize)>> = BTreeMap::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell != b'.' {
                antennas.entry(cell).or_default().push((x, y));
            }
        }
    }

    let mut antis = BTreeSet::<(usize, usize)>::new();
    for (_kind, pos) in antennas {
        for (i, (x1, y1)) in pos.iter().copied().enumerate() {
            for (x2, y2) in pos.iter().copied().skip(i + 1) {
                let dx = x1 as isize - x2 as isize;
                let dy = y1 as isize - y2 as isize;

                let a1x = x1 as isize + dx;
                let a2x = x2 as isize - dx;
                let a1y = y1 as isize + dy;
                let a2y = y2 as isize - dy;

                if (0..bound_x).contains(&a1x) && (0..bound_y).contains(&a1y) {
                    antis.insert((a1x as usize, a1y as usize));
                }

                if (0..bound_x).contains(&a2x) && (0..bound_y).contains(&a2y) {
                    antis.insert((a2x as usize, a2y as usize));
                }
            }
        }
    }

    Ok(antis.len())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let bound_y = input.len() as isize;
    let bound_x = input[0].len() as isize;

    let mut antennas: BTreeMap<u8, Vec<(usize, usize)>> = BTreeMap::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell != b'.' {
                antennas.entry(cell).or_default().push((x, y));
            }
        }
    }

    let mut antis = BTreeSet::<(usize, usize)>::new();
    for (_kind, pos) in antennas {
        for (i, (x1, y1)) in pos.iter().copied().enumerate() {
            for (x2, y2) in pos.iter().copied().skip(i + 1) {
                let dx = x1 as isize - x2 as isize;
                let dy = y1 as isize - y2 as isize;

                // backwards
                let mut ax = x1 as isize;
                let mut ay = y1 as isize;
                while (0..bound_x).contains(&ax) && (0..bound_y).contains(&ay) {
                    antis.insert((ax as usize, ay as usize));
                    ax -= dx;
                    ay -= dy;
                }

                // forwards
                let mut ax = x2 as isize;
                let mut ay = y2 as isize;
                while (0..bound_x).contains(&ax) && (0..bound_y).contains(&ay) {
                    antis.insert((ax as usize, ay as usize));
                    ax += dx;
                    ay += dy;
                }
            }
        }
    }

    Ok(antis.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 14 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 34 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
