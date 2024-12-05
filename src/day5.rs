// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;

type Answer = usize;

type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);
fn munge_input(input: &str) -> DynResult<Input> {
    let Some((rules, orders)) = input.split_once("\n\n") else {
        todo!()
    };

    let rules = rules
        .split('\n')
        .map(|x| x.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect();

    let orders = orders
        .split('\n')
        .map(|s| s.split(',').map(|x| x.parse::<usize>().unwrap()).collect())
        .collect();

    Ok((rules, orders))
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (rules, orders) = munge_input(input)?;

    let mut m = BTreeMap::<usize, BTreeSet<usize>>::new();
    for (before, after) in rules {
        m.entry(before).or_default().insert(after);
    }

    // brute force this thing
    let mut total = 0;
    'outer: for order in orders {
        for i in 0..order.len() {
            if i + 1 == order.len() {
                break;
            }

            let pb = order[i];
            let Some(rules) = m.get(&pb) else {
                continue 'outer;
            };

            for j in (i + 1)..order.len() {
                let pa = order[j];
                if !rules.contains(&pa) {
                    continue 'outer;
                }
            }
        }

        total += order[order.len() / 2];
    }

    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let (rules, orders) = munge_input(input)?;

    let mut all = BTreeSet::new();

    let mut before_to_after = BTreeMap::<usize, BTreeSet<usize>>::new();
    let mut after_to_before = BTreeMap::<usize, BTreeSet<usize>>::new();
    for (before, after) in rules {
        all.insert(before);
        all.insert(after);

        before_to_after.entry(after).or_default();
        before_to_after.entry(before).or_default().insert(after);

        after_to_before.entry(before).or_default();
        after_to_before.entry(after).or_default().insert(before);
    }

    dbg!();
    dbg_map!(&before_to_after);
    dbg!();
    dbg_map!(&after_to_before);

    let mut total = 0;
    for order in orders {
        let ignored = {
            let mut s = all.clone();
            for x in &order {
                s.remove(x);
            }
            s
        };

        let mut before_to_after: BTreeMap<_, BTreeSet<_>> = before_to_after
            .clone()
            .into_iter()
            .map(|(x, s)| (x, s.difference(&ignored).copied().collect()))
            .collect();
        let mut after_to_before: BTreeMap<_, BTreeSet<_>> = after_to_before
            .clone()
            .into_iter()
            .map(|(x, s)| (x, s.difference(&ignored).copied().collect()))
            .collect();

        let root = *order
            .iter()
            .find(|x| before_to_after.get(x).unwrap().is_empty())
            .unwrap();

        // do toposort
        let mut out = Vec::new();
        let mut s = BTreeSet::from([root]);
        while let Some(n) = s.pop_first() {
            out.push(n);
            for m in after_to_before.get(&n).unwrap().clone() {
                let me = before_to_after.get_mut(&m).unwrap();
                me.remove(&n);
                if me.is_empty() {
                    s.insert(m);
                }
            }
        }

        out.reverse();

        if out == order {
            continue;
        }

        dbg!(&root);
        dbg!(&out);
        total += out[out.len() / 2];
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 143 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 123 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
