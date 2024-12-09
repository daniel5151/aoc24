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

    // maintain a min-heap of gaps, ordered by length, with (len, gap-idx)
    let mut gaps = BinaryHeap::<Reverse<(usize, usize)>>::new();
    // maintain a vec of files, where pos is the file ID, and entries are simply
    // (file-idx)
    let mut files = Vec::<usize>::new();

    for (idx, n) in input.into_iter().enumerate() {
        if idx % 2 == 0 {
            files.push(idx);
        } else {
            gaps.push(Reverse((n as usize, idx)));
        }
    }

    // vec of (file_id, file_idx, len)
    let mut final_disk = Vec::<(usize, usize, usize)>::new();
    for (file_id, file_idx) in files.into_iter().enumerate().rev() {
        //
    }

    let mut sum = 0;
    for (file_id, file_idx, len) in final_disk {
        for i in file_idx..(file_id + len) {
            sum += file_id * i;
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
