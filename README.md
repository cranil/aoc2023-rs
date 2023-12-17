# aoc2023-rs
[Advent of Code 2023](https://adventofcode.com/2023)

Runtimes
===
Runtimes on a Macbook Air (M1, 16G)

``` text

┌────┬────────┬────────────┐
│Day │  Part  │ Time (us)  │
├────┼────────┼────────────┤
│ 1  │   1    │    25.4    │
├────┼────────┼────────────┤
│ 1  │   2    │   120.3    │
├────┼────────┼────────────┤
│ 2  │   1    │    56.1    │
├────┼────────┼────────────┤
│ 2  │   2    │     47     │
├────┼────────┼────────────┤
│ 3  │   1    │    35.1    │
├────┼────────┼────────────┤
│ 3  │   2    │   153.8    │
├────┼────────┼────────────┤
│ 4  │   1    │    26.1    │
├────┼────────┼────────────┤
│ 4  │   2    │    28.3    │
├────┼────────┼────────────┤
│ 5  │   1    │    2.5     │
├────┼────────┼────────────┤
│ 5  │   2    │167078748.4 │
├────┼────────┼────────────┤
│ 6  │   1    │     0      │
├────┼────────┼────────────┤
│ 6  │   2    │    0.6     │
├────┼────────┼────────────┤
│ 7  │   1    │   319.4    │
├────┼────────┼────────────┤
│ 7  │   2    │   506.9    │
├────┼────────┼────────────┤
│ 8  │   1    │    2488    │
├────┼────────┼────────────┤
│ 8  │   2    │  13625.7   │
├────┼────────┼────────────┤
│ 9  │   1    │     28     │
├────┼────────┼────────────┤
│ 9  │   2    │    45.5    │
├────┼────────┼────────────┤
│ 10 │   1    │   1156.3   │
├────┼────────┼────────────┤
│ 10 │   2    │   2599.3   │
├────┼────────┼────────────┤
│ 11 │   1    │   713.8    │
├────┼────────┼────────────┤
│ 11 │   2    │   712.4    │
├────┼────────┼────────────┤
│ 12 │   1    │   6549.8   │
├────┼────────┼────────────┤
│ 12 │   2    │  134396.5  │
├────┼────────┼────────────┤
│ 13 │   1    │    10.2    │
├────┼────────┼────────────┤
│ 13 │   2    │    20.3    │
├────┼────────┼────────────┤
│ 14 │   1    │    57.6    │
├────┼────────┼────────────┤
│ 14 │   2    │  31579.2   │
├────┼────────┼────────────┤
│ 15 │   1    │    47.3    │
├────┼────────┼────────────┤
│ 15 │   2    │   532.7    │
├────┼────────┼────────────┤
│ 16 │   1    │   178.1    │
├────┼────────┼────────────┤
│ 16 │   2    │  49656.8   │
├────┼────────┼────────────┤
│ 17 │   1    │  54715.4   │
├────┼────────┼────────────┤
│ 17 │   2    │  126157.5  │
└────┴────────┴────────────┘
```
Usage
===
```
# cargo run --release -- --help
Usage: aoc2023 [OPTIONS]

Options:
  -d, --day <DAY>            [default: 0]
  -p, --part <PART>          [default: 0]
  -n, --num-runs <NUM_RUNS>  [default: 1]
  -h, --help                 Print help
  -V, --version              Print version

# cargo test --release day05

```
