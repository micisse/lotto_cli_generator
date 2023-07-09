use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = None)]
struct Args {
    #[arg(long, required = false)]
    input_1: Option<String>, // numbers

    #[arg(long, required = false)]
    input_2: Option<String>, // odds numbers

    #[arg(long, default_value_t = 10, required = false)]
    grid_count: i32,
}

fn main() {
    let args = Args::parse();
    let grid_count = args.grid_count; // Numbers of grid to generate
    let mut combinaisons: Vec<Vec<i32>> = Vec::new();
    let grid_numbers: Vec<i32> = (1..=49).collect(); // Each grid (odds numbers) have 49 numbers
    let grid_odds_numbers: Vec<i32> = (1..=10).collect(); // Each grid (odds numbers) have 10 numbers
    let input_1 = match args.input_1 {
        Some(value) => collect_input(value),
        None => grid_numbers,
    };
    let input_2 = match args.input_2 {
        Some(value) => collect_input(value),
        None => grid_odds_numbers,
    };

    if input_1.len() < 5 || input_1.len() > 5 {
        panic!("'input_1' doit obligatoirement contenir {} nombres", 5);
    }

    for idx in 1..=grid_count {
        let numbers = generate_new_numbers(input_1.clone(), 5);
        let odds_numbers = generate_new_numbers(input_2.clone(), 1);

        combinaisons.push(numbers.clone()); // Push each combinaison in new vec
        println!(
            "Combinaison {}: Numéros: {:?}, Chance: {:?}",
            idx, numbers, odds_numbers
        );
    }

    println!("Combinaisons {:?}", combinaisons);
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
