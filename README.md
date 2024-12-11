# Advent of Code 2024

My solutions to Advent of Code 2024.

I'm once again recycling the same home-grown harness I hacked together in
previous years.

Will I actually stick with it this year? Unclear! Especially since I'm going on
vacation on Dec 11th.

**Update:** Its December 10th, and look at that - I actually did 11 days of
questions! But now its time for vacation, and I won't have a laptop, so it seems
Day 11 is the end of the road for me this year...

## Running

(Assuming you've put the desired day's input into the `inputs` dir, either manually, or via `run.sh`)

```bash
cargo run --release -- <day> <question>
```

Tests can be run using the standard `cargo test` flow.

```bash
cargo test -- dayX # only runs tests for the particular day
```

## Running (on the day of)

Manually copying question input? Nahhhh, we can do better than that.

When tests are passing and you're ready for prime-time, skip `cargo` and use the `run.sh` script:

```bash
./run.sh <day> <question>
# e.g: ./run 3 1
```

The harness will automatically download question input if a `cookie.txt` is provided.

`cookie.txt`'s should contain the following string:

```
session=53616c...
```

Getting `cookie.txt` is easy:
- Open Chrome
- Navigate to _any_ day's input URL (e.g: https://adventofcode.com/2024/day/1/input)
- Open the Chrome Network Inspector
- Refresh the URL
- Right click the `input` request, and "copy > copy as cURL"
    - the string should include a `-H 'cookie: <cookie.txt>'` component.
