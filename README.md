<img src="./.assets/rust-banner.jpg" width="500">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2024 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2024/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2024/day/4) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->

[Template from fspoettel](https://github.com/fspoettel/advent-of-code-rust)

## TL;DR

| **Command**                   | **Usage**                                                                                 |
|-------------------------------|-------------------------------------------------------------------------------------------|
| `cargo scaffold <day>`        | Creates module, input, and example files for a day.                                       |
| `cargo download <day>`        | Downloads puzzle input and description for a day.                                         |
| `cargo solve <day> [--submit <part>]`| Runs solution for a specific day with real inputs. Use `--submit <part>` to submit the answer to the website  |
| `cargo all`                   | Runs all solutions sequentially.                                                          |
| `cargo time <day> [--store]`  | Benchmarks code for a day (use `--store` to save results).                                 |
| `cargo test`                  | Runs all tests (use `--bin <day>` for specific day).                                       |
| `cargo read <day>`            | Fetches and displays puzzle description for a day.                                        |
| `cargo today`                 | Scaffolds, downloads, and reads the puzzle for the current day in December.               |
| `cargo fmt`                   | Formats all code.                                                                         |
| `cargo clippy`                | Lints all code.                                                                           |
---

## 💻 Requierments

1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2. Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.12.0`
2. Create the file `<home_directory>/.adventofcode.session` and paste your session cookie into it.

## Usage

### ➡️ Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/template.txt) has _tests_ referencing its _example_ file in `./data/examples`. Use these tests to develop and debug your solutions against the example input.

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an arbitrary number of example files.

### ➡️ Download input for a day

> [!IMPORTANT]
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

You can automatically download puzzle input and description by either appending the `--download` flag to `scaffold` (e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

#### Submitting solutions

> [!IMPORTANT]
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

Append the `--submit <part>` option to the `solve` command to submit your solution for checking.

### ➡️ Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

### ➡️ Benchmark your solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner will run your code between `10` and `10.000` times, depending on execution time of first execution, and print the average execution time.

`cargo time` has three modes of execution:

 1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and skips the rest.
 2. `cargo time <day>` benches a single solution.
 3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag: `cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### ➡️ Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### ➡️ Read puzzle description

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Scaffold, download & read the current aoc day

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

During december, the `today` shorthand command can be used to:

 - scaffold a solution for the current day
 - download its input
 - and read the puzzle

in one go.

```sh
# example: `cargo today` on December 1st
cargo today

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
#
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Format code

```sh
cargo fmt
```

### ➡️ Lint code

```sh
cargo clippy
```
