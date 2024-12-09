// lints that are loud when speedrunning. removed before commit
#![allow(unused_mut, clippy::let_and_return)]

use crate::prelude::*;
use std::cmp::Reverse;

type Answer = usize;

type Input = Vec<u8>;
fn munge_input(input: &str) -> DynResult<Input> {
    Ok(input.as_bytes().iter().map(|&x| x - b'0').collect())
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let mut disk = Vec::new();
    for (idx, n) in input.into_iter().enumerate() {
        if idx % 2 == 0 {
            disk.extend(std::iter::repeat_n(idx / 2, n as usize));
        } else {
            disk.extend(std::iter::repeat_n(usize::MAX, n as usize));
        }
    }

    let mut end_idx = disk.len() - 1;
    let mut start_idx = 0;
    while start_idx < end_idx {
        if disk[start_idx] == usize::MAX {
            disk[start_idx] = disk[end_idx];
            disk[end_idx] = usize::MAX;
            end_idx -= 1;
            while disk[end_idx] == usize::MAX && end_idx != 0 {
                end_idx -= 1;
            }

            if end_idx == 0 {
                break;
            }
        }
        start_idx += 1;
    }

    // checksum
    let mut sum = 0;
    for (i, n) in disk
        .into_iter()
        .take_while(|&x| x != usize::MAX)
        .enumerate()
    {
        sum += i * n;
    }

    Ok(sum)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<Answer> {
    let mut input = munge_input(input)?;

    let mut gap_idx_by_size: [BinaryHeap<Reverse<usize>>; 10] = [const { BinaryHeap::new() }; 10];
    let mut file_id_to_idx_len = Vec::<(usize, usize)>::new();

    let mut idx = 0;
    for (kind, n) in input.into_iter().enumerate().map(|(i, b)| (i % 2, b)) {
        if n == 0 {
            // files can't be zero-len
            if kind == 0 {
                panic!()
            }

            continue;
        }

        if kind == 0 {
            file_id_to_idx_len.push((idx, n as usize));
        } else {
            gap_idx_by_size[n as usize].push(Reverse(idx));
        }

        idx += n as usize;
    }

    fn debug_disk(disk: &[(usize, usize, usize)]) {
        let mut viz = Vec::new();
        for &(file_id, file_idx, len) in disk {
            if file_idx + len > viz.len() {
                viz.resize(file_idx + len, 1000);
            }
            for i in file_idx..file_idx + len {
                viz[i] = file_id;
            }
        }

        println!("{viz:?}");
    }

    // vec of (file_id, file_idx, len)
    let mut final_disk = Vec::<(usize, usize, usize)>::new();
    for (file_id, (file_idx, file_len)) in file_id_to_idx_len.into_iter().enumerate().rev() {
        let mut candidates = Vec::new();

        // figure out the best gaps in each len class for this file
        for (gap_len, gaps) in gap_idx_by_size.iter_mut().enumerate().skip(file_len) {
            let Some(&Reverse(candidate_idx)) = gaps.peek() else {
                continue;
            };

            if candidate_idx > file_idx {
                continue;
            }

            // cool, this is a viable gap
            candidates.push((candidate_idx, gap_len));
        }

        // find _left-most_ gap where we can put the file (not strictly the smallest!)
        candidates.sort();
        if let Some(&(candidate_idx, gap_len)) = candidates.first() {
            gap_idx_by_size[gap_len].pop();
            let new_gap_len = gap_len - file_len;
            if new_gap_len != 0 {
                let new_gap_idx = candidate_idx + file_len;
                gap_idx_by_size[new_gap_len].push(Reverse(new_gap_idx));
            }

            final_disk.push((file_id, candidate_idx, file_len));
        } else {
            //  file stays put
            final_disk.push((file_id, file_idx, file_len));
        }
    }

    debug_disk(&final_disk);

    let mut sum = 0;
    for (id, idx, len) in final_disk {
        for i in idx..(idx + len) {
            sum += id * i;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
2333133121414131402
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 1928 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 2858 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
