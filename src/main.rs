mod utils;

use crate::utils::generate_new_numbers;
use clap::Parser;
use std::{thread, time::Duration};
use utils::collect_input;

#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = "Lotto combination CLI generator. Enter 5 or more numbers and/or 1 or more lucky numbers as arguments or generate default combinations between 1 and 49 with random grids of 5 numbers and 1 lucky number.")]
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

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let arg_grid_count = args.grid_count; // Numbers of grid to generate
    let arg_mix: bool = args.mix; // Mix the result
    let arg_numbers = args.numbers;
    let arg_odds_numbers = args.odds_numbers;
    let mut combinaisons: Vec<Vec<i32>> = Vec::new();
    let mut odds_combinaisons: Vec<Vec<i32>> = Vec::new();
    let grid_numbers: Vec<i32> = (1..=49).collect(); // Each grid have 49 numbers
    let grid_odds_numbers: Vec<i32> = (1..=10).collect(); // Each grid (odds numbers) have 10 numbers

    // let fmt = "%A %d %B %Y";
    // let locale = Locale::fr_FR;
    // let cur_date = Local::now().format_localized(fmt, locale);
    // let cur_hour = Local::now().format("%H:%M");
    // let curr_exe = env::current_exe().unwrap();
    // let full_path = String::from(curr_exe.to_str().unwrap());
    // let path_split: Vec<&str> = full_path.split("/target/release/lotto_generator").collect();
    // let current_dir = path_split[0];
    // let output_path = format!("{}/Tirage du {} à {}.txt", current_dir, cur_date, cur_hour);
    // let _created_path = File::create(&output_path)?;
    // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
    // let mut output_file = OpenOptions::new().append(true).open(output_path)?;

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

    numbers.sort();
    numbers.dedup();
    odds_numbers.sort();
    odds_numbers.dedup();

    // if arg_numbers.is_some() {
    //     let line_to_file = format!(
    //         "╰─ CMD: lotto_generator -n '{}' --odds-numbers '{}' --grid-count {} --mix\n\n",
    //         arg_numbers.unwrap(),
    //         arg_odds_numbers.unwrap(),
    //         arg_grid_count
    //     );

    //     output_file.write(line_to_file.as_bytes())?;
    // }

    'loop_label: for idx in 1..=arg_grid_count {
        let i = idx - 1;

        if arg_mix && i >= (arg_grid_count / 2) {
            numbers = grid_numbers.clone();
            odds_numbers = grid_odds_numbers.clone();
        };

        let new_numbers = generate_new_numbers(&numbers, 5);
        let new_odds_numbers = generate_new_numbers(&odds_numbers, 1);
        let odd_number = new_odds_numbers[0];
        let odd_index = odds_numbers.iter().position(|&r| r == odd_number).unwrap();

        if combinaisons.contains(&new_numbers) {
            println!("\n----| SKiP\n");
            thread::sleep(Duration::from_millis(1000));
            let _ = main();
            break 'loop_label;
        }

        combinaisons.push(new_numbers.to_owned());
        odds_combinaisons.push(new_odds_numbers.to_owned());

        if odds_numbers.len() > 1 {
            odds_numbers.remove(odd_index);
        }

        // let line_to_file = format!(
        //     "Numbers: {:?}, Odd number: {} | Numéros gagnés: '<À saisir après tirage>' \n\n",
        //     new_numbers, odd_number
        // );
        let line_to_terminal = format!("Numbers: {:?}, Odd number: {}", new_numbers, odd_number);

        println!("\n----| Combination N°{} = {}", idx, line_to_terminal);
        // output_file.write(line_to_file.as_bytes())?;
    }

    Ok(())
}
