use crate::prelude::*;

type Answer = usize;

type Input = (Vec<usize>, Vec<usize>);
fn munge_input(input: &str) -> DynResult<Input> {
    input
        .split('\n')
        .map(|s| -> DynResult<_> {
            let res = {
                let (a, b) = s.split_once("   ").ok_or("missing space")?;
                (a.parse::<usize>()?, b.parse::<usize>()?)
            };
            Ok(res)
        })
        .process_results(|input| input.unzip())
        .map_err(Into::into)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (a, b) = munge_input(input)?;

    // doesn't need to be a min-heap, since there's no real diff between
    // iterating from smallest-to-largest vs. largest-to-smallest.
    let mut a = a.into_iter().collect::<BinaryHeap<_>>();
    let mut b = b.into_iter().collect::<BinaryHeap<_>>();

    let mut total = 0;
    while let (Some(a), Some(b)) = (a.pop(), b.pop()) {
        total += a.abs_diff(b)
    }

    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (a, b) = munge_input(input)?;

    let b_occurs = b
        .into_iter()
        .fold(BTreeMap::<usize, usize>::new(), |mut a, x| {
            *a.entry(x).or_default() += 1;
            a
        });

    // alternatively, using itertools
    // let b_occurs = b.into_iter().counts();

    let mut total = 0;
    for x in a {
        total += x * b_occurs.get(&x).cloned().unwrap_or_default();
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 11 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 31 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
