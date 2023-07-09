use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};
use std::{thread, time::Duration};

#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = None)]
struct Args {
    /// Numbers list
    #[arg(short, long = "numbers", required = false)]
    numbers: Option<String>,
    /// Odds numbers list
    #[arg(long = "odds-numbers", required = false)]
    odds_numbers: Option<String>,
    /// Numbers of grid to generate
    #[arg(long = "grid-count", default_value_t = 10, required = false)]
    grid_count: i32,
    /// Type of combination generation
    ///
    /// **false**: Generate from argument "numbers" pass as parameter
    ///
    /// **true**: Generate from argument list "numbers" and total number grid content (1..=49)
    ///
    /// Generate from the "numbers" argument and the numbers included in each grid (1..=49) by dividing the number
    /// of grids (grid_count) by 2. If 10, then 10 / 2 = 5, so 5 grids with combinations based on
    /// "numbers" argument and another 5 grids with the numbers in each grid.
    #[arg(long = "mix", default_value_t = false, required = false)]
    mix: bool,
}

fn main() {
    let args = Args::parse();
    let arg_grid_count = args.grid_count; // Numbers of grid to generate
    let arg_mix: bool = args.mix; // Mix the result
    let arg_numbers = args.numbers;
    let arg_odds_numbers = args.odds_numbers;
    let mut combinaisons: Vec<Vec<i32>> = Vec::new();
    let mut odds_combinaisons: Vec<Vec<i32>> = Vec::new();
    let grid_numbers: Vec<i32> = (1..=49).collect(); // Each grid have 49 numbers
    let grid_odds_numbers: Vec<i32> = (1..=10).collect(); // Each grid (odds numbers) have 10 numbers

    let mut numbers = match arg_numbers.to_owned() {
        Some(value) => collect_input(value),
        None => grid_numbers.to_owned(),
    };
    let mut odds_numbers = match arg_odds_numbers.to_owned() {
        Some(value) => collect_input(value),
        None => grid_odds_numbers.to_owned(),
    };

    if arg_mix && arg_numbers.is_none() {
        panic!("'numbers' argument is required");
    }

    'loop_label: for idx in 1..=arg_grid_count {
        let i = idx - 1;
        let mut category = 1;

        if arg_mix && i >= (arg_grid_count / 2) {
            numbers = grid_numbers.clone();
            odds_numbers = grid_odds_numbers.clone();
            category += 1;
        };

        let new_numbers = generate_new_numbers(&numbers, 5);
        let new_odds_numbers = generate_new_numbers(&odds_numbers, 1);
        let odd_number = new_odds_numbers[0];
        let odd_index = odds_numbers.iter().position(|&r| r == odd_number).unwrap();

        if combinaisons.contains(&new_numbers) {
            println!("\n----| SKiP\n");
            thread::sleep(Duration::from_millis(1000));
            main();
            break 'loop_label;
        }

        combinaisons.push(new_numbers.to_owned());
        odds_combinaisons.push(new_odds_numbers.to_owned());

        if odds_numbers.len() > 1 {
            odds_numbers.remove(odd_index);
        }

        println!("----| Category {} | Combination n°{}", category, idx);
        println!("Numbers: {:?}, Odd number: {}\n", new_numbers, odd_number);
    }
}

fn collect_input(input: String) -> Vec<i32> {
    input
        .split(&[' ', ','][..])
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn generate_new_numbers(input: &Vec<i32>, take_nbr: usize) -> Vec<i32> {
    let mut cloned_input = input.to_owned();

    cloned_input.shuffle(&mut thread_rng());
    cloned_input
        .iter()
        .take(take_nbr)
        .map(|x| *x)
        .collect::<Vec<i32>>()
}
