use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};

// https://docs.rs/clap/latest/clap/index.html
#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = None)]
struct Args {
    #[arg(long, required = true)]
    input_1: String, // numéros normales
    #[arg(long, required = true)]
    input_2: String, // numéros chances

    #[arg(long, default_value_t = 10, required = false)]
    grid_count: i32,
}

fn main() {
    let args = Args::parse();
    let grid_count = args.grid_count; // Le nombre de grille à générer
    let _lotto_price = 2.20; // €
    let _grid_numbers: Vec<i32> = (1..=49).collect(); // Chaque grille contient 49 nombres
    let _grid_chance_numbers: Vec<i32> = (1..=10).collect(); // Chaque grille (côté numéro chance) contient 10 nombres
    let input_1 = collect_input(args.input_1 as String);
    let input_2 = collect_input(args.input_2 as String);

    if input_1.len() < 5 || input_1.len() > 5 {
        panic!("'input_1' doit obligatoirement contenir {} nombres", 5);
    }

    for idx in 1..=grid_count {
        let numbers = generate_new_numbers(input_1.clone(), 5);
        let odds_numbers = generate_new_numbers(input_2.clone(), 5);

        println!(
            "Combinaison {}: Numéros: {:?}, Chance: {:?}",
            idx, numbers, odds_numbers
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
