# aoc2023-rs
[Advent of Code 2023](https://adventofcode.com/2023)

Runtimes
===
Runtimes on a Macbook Air (M1, 16G)

``` text

┌────┬────────┬────────────────────────────────┬────────────┐
│Day │  Part  │             Result             │ Time (us)  │
├────┼────────┼────────────────────────────────┼────────────┤
│ 1  │   1    │             54450              │    25.4    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 1  │   2    │             54265              │   120.3    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 2  │   1    │              2716              │    56.1    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 2  │   2    │             72227              │     47     │
├────┼────────┼────────────────────────────────┼────────────┤
│ 3  │   1    │             553079             │    35.1    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 3  │   2    │            84363105            │   153.8    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 4  │   1    │             24175              │    26.1    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 4  │   2    │            18846301            │    28.3    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 5  │   1    │            3374647             │    2.5     │
├────┼────────┼────────────────────────────────┼────────────┤
│ 5  │   2    │            6082852             │167078748.4 │
├────┼────────┼────────────────────────────────┼────────────┤
│ 6  │   1    │             588588             │     0      │
├────┼────────┼────────────────────────────────┼────────────┤
│ 6  │   2    │            34655848            │    0.6     │
├────┼────────┼────────────────────────────────┼────────────┤
│ 7  │   1    │           247815719            │   319.4    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 7  │   2    │           248747492            │   506.9    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 8  │   1    │             19099              │    2488    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 8  │   2    │         17099847107071         │  13625.7   │
├────┼────────┼────────────────────────────────┼────────────┤
│ 9  │   1    │           1479011877           │     28     │
├────┼────────┼────────────────────────────────┼────────────┤
│ 9  │   2    │              973               │    45.5    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 10 │   1    │              6846              │   1156.3   │
├────┼────────┼────────────────────────────────┼────────────┤
│ 10 │   2    │              325               │   2599.3   │
├────┼────────┼────────────────────────────────┼────────────┤
│ 11 │   1    │            9565386             │   713.8    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 11 │   2    │          857986849428          │   712.4    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 12 │   1    │              7195              │   6549.8   │
├────┼────────┼────────────────────────────────┼────────────┤
│ 12 │   2    │         33992866292225         │  134396.5  │
├────┼────────┼────────────────────────────────┼────────────┤
│ 13 │   1    │             36041              │    10.2    │
├────┼────────┼────────────────────────────────┼────────────┤
│ 13 │   2    │             35915              │    20.3    │
└────┴────────┴────────────────────────────────┴────────────┘
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
