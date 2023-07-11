# LOTTO CLI GENERATOR

### PREREQUISITES

- **Rust**: *1.70.0* (with cargo)
- **Rust Analyzer (dev & optional)**: *0.3.1583-standalone or ...*

### ABOUT

**Lotto combination CLI generator**. Enter 5 or more numbers and/or 1 or more lucky numbers as arguments or generate default combinations between 1 and 49 with random grids of 5 numbers and 1 lucky number.

#### Help

```sh
╰─ cargo run -- --help
╰─ cargo run -- --h
```
![lotto_cli_generator](https://github.com/micisse/lotto_cli_generator/assets/56940294/5b221796-e96e-4528-93c6-9c994e3353de)

#### Dev mode

```sh
cargo run -- -n "6 10 19 32 48 17 33 39 41" --odds-numbers "10 4 9 5" --grid-count 4 --mix
```

```sh
cargo run -- --numbers "<numbers list>" --odds-numbers "<odds number list>" --grid-count <GRID_COUNT> --mix
```

#### Release mode

```sh
cargo build --release
```
```sh
./target/release/lotto_cli_generator --numbers "<numbers list>" --odds-numbers "<odds number list>" --grid-count <GRID_COUNT> --mix <mix is BOOLEAN, true if present, false if not>
```
