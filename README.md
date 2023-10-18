# CLI LOTTO COMBINATION GENERATOR

### PREREQUISITES

- **rust**: *1.73.0* (with cargo)
- **cargo**: *1.73.0*
- **rust-analyzer (dev & optional)**: *0.3.1697-standalone or ...*

### ABOUT

Generate grid combinations. Enter 5 or more numbers and/or 1 or more lucky numbers as arguments or generate default combinations between 1 and 49 with random grids of 5 numbers and 1 lucky number.

#### Help

```sh
╰─ cargo run -- --help
╰─ cargo run -- --h
```

#### Dev mode

```sh
cargo run -- -n "6 10 19 32 48 17 33 39 41" --odds-numbers "10 4 9 5" --grid-count 4 --mix "false false false true"
```

```sh
cargo run -- --numbers "<NUMBERS>" --odds-numbers "<ODDS_NUMBER>" --grid-count <GRID_COUNT> [--mix "<BOOL> <BOOL> <BOOL> <BOOL>"]
```

#### Release mode

```sh
cargo build --release
```
```sh
./target/release/lotto_cli_generator --numbers "<NUMBERS>" --odds-numbers "<ODDS_NUMBER>" --grid-count <GRID_COUNT> [--mix "<BOOL> <BOOL> <BOOL> <BOOL>"]
```

#### Constraints & notes

***The zero (0) does not exist in any of these games. It is therefore unnecessary to use the zero in the arguments. It all starts with 1.***

- **--numbers** is the data list. The data to search in.

- **--odds-numbers** is the second data list. Chance number data to search in.

- **--grid-count** is the number of grid to generate.

- **--mix** is used to mix output. (optional)
  - **absent**: Generate result only from data list
  - **present**: Generate from data list and total number of grid content. Every game has maximum limit of numbers.
  - i.e: If **present**, generate from data list and the numbers included in each grid by dividing the number of grids (**--grid-count**) by 2. If **--grid-count = 10**, then 10 / 2 = 5, so 5 grids with combinations based on data list and another 5 grids with the default numbers in grid.
  - We can define a single mix in **--mix** at position 1 or higher (i.e.: **--mix = "false"** or **--mix = "true false"**"), in this case, other grids without assigned values will be set to false by default.
  - Cannot be greater than **--grid-count**. It can be only smaller or equal to **--grid-count**.
  - i.e: if **--grid-count = 2, --mix** must be like that **"Bool1 Bool2"**
  - i.e: if **--grid-count = 6, --mix** must be like that **"Bool1 Bool2 Bool3 Bool4 Bool5 Bool6"**
  - A grid is represented in **--mix** by its position "**Grid1 Grid2 Grid3 Grid4 Grid5 Grid6**".