use crate::prelude::*;

type Answer = usize;

type Input = Vec<Vec<u8>>;
fn munge_input(input: &str) -> DynResult<Input> {
    let input = input.split('\n').map(|s| s.as_bytes().to_vec()).collect();
    Ok(input)
}

fn solve(input: &str, harmonics: bool) -> DynResult<Answer> {
    let input = munge_input(input)?;

    let bound_y = input.len() as isize;
    let bound_x = input[0].len() as isize;

    let mut antennas: BTreeMap<u8, Vec<(isize, isize)>> = BTreeMap::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell != b'.' {
                antennas
                    .entry(cell)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let mut antis = BTreeSet::<(isize, isize)>::new();
    for (_kind, pos) in antennas {
        for (i, (x1, y1)) in pos.iter().copied().enumerate() {
            for (x2, y2) in pos.iter().copied().skip(i + 1) {
                let dx = x1 - x2;
                let dy = y1 - y2;

                for (dir, (mut ax, mut ay)) in [(1, (x1, y1)), (-1, (x2, y2))] {
                    let mut i = 0;
                    while (0..bound_x).contains(&ax) && (0..bound_y).contains(&ay) {
                        let pos = (ax, ay);

                        i += 1;
                        ax += dx * dir;
                        ay += dy * dir;

                        if !harmonics {
                            match i {
                                1 => continue,
                                2 => {}
                                3 => break,
                                _ => unreachable!(),
                            }
                        }

                        antis.insert(pos);
                    }
                }
            }
        }
    }

    Ok(antis.len())
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    solve(input, false)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    solve(input, true)
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
