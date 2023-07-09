use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};
use std::{thread, time::Duration};

#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = None)]
struct Args {
    /// Numbers list
    #[arg(long = "numbers", required = false)]
    numbers: Option<String>,

    /// Odds numbers list
    #[arg(long = "odds-numbers", required = false)]
    odds_numbers: Option<String>,

    /// Numbers of grid to generate
    #[arg(long = "grid-count", default_value_t = 10, required = false)]
    grid_count: i32,
}

fn main() {
    let args = Args::parse();
    let grid_count = args.grid_count; // Numbers of grid to generate
    let mut combinaisons: Vec<Vec<i32>> = Vec::new();
    let mut odds_combinaisons: Vec<Vec<i32>> = Vec::new();
    let grid_numbers: Vec<i32> = (1..=49).collect(); // Each grid have 49 numbers
    let grid_odds_numbers: Vec<i32> = (1..=10).collect(); // Each grid (odds numbers) have 10 numbers

    let numbers = match args.numbers {
        Some(value) => collect_input(value),
        None => grid_numbers,
    };
    let mut odds_numbers = match args.odds_numbers {
        Some(value) => collect_input(value),
        None => grid_odds_numbers,
    };

    if numbers.len() < 5 {
        panic!("'numbers' must contain at least {} numbers", 5);
    }
    if odds_numbers.len() < 1 {
        panic!("'odds_numbers' must contain at least {} numbers", 1);
    }

    'numbers: for idx in 1..=grid_count {
        let new_numbers = generate_new_numbers(numbers.clone(), 5);
        let new_odds_numbers = generate_new_numbers(odds_numbers.clone(), 1);
        let odd_number = new_odds_numbers[0];
        let index = odds_numbers.iter().position(|&r| r == odd_number).unwrap();

        if combinaisons.contains(&new_numbers) {
            println!("\n----|SKiP\n");
            thread::sleep(Duration::from_millis(1000));
            main();
            break 'numbers;
        }

        combinaisons.push(new_numbers.clone());
        odds_numbers.remove(index);
        odds_combinaisons.push(new_odds_numbers.clone());
        println!(
            "Combination n°{} | Numbers: {:?}, Odd number: {}",
            idx, new_numbers, odd_number
        );
    }
}

fn collect_input(input: String) -> Vec<i32> {
    input
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn generate_new_numbers(input: Vec<i32>, take_nbr: usize) -> Vec<i32> {
    let mut cloned_input = input.clone();

    cloned_input.shuffle(&mut thread_rng());
    cloned_input
        .iter()
        .take(take_nbr)
        .map(|x| *x)
        .collect::<Vec<i32>>()
}
