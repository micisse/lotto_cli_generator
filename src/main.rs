use clap::Parser;

// https://docs.rs/clap/latest/clap/index.html
#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = None)]
struct Args {
    #[arg(long, required = true)]
    input_1: String, // numéros normales
    #[arg(long, required = true)]
    input_2: String, // numéros chances

    #[arg(long, default_value_t = 10, required = false)]
    grid_count: i8,
}

fn main() {
    // let lotto_price = 2.20; // €
    // Chaque grille contient 49 nombres
    let grid_numbers: Vec<i32> = (1..=49).collect();
    // Chaque grille (côté numéro chance) contient 10 nombres
    // let grid_chance_numbers: Vec<i32> = (1..=10).collect();

    let args = Args::parse();
    // Le nombre de grille à générer
    let grid_count: i8 = args.grid_count;

    let input_1_args: String = args.input_1;
    let input_1 = input_1_args
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // let input_2_args: String = args.input_2;
    // let input_2 = input_2_args
    //     .split(" ")
    //     .collect::<Vec<&str>>()
    //     .iter()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect::<Vec<i32>>();

    if input_1.len() < 5 || input_1.len() > 5 {
        panic!("'input_1' doit obligatoirement contenir {} nombres", 5);
    }

    println!("{:?}", input_1);
    println!("{:?}", grid_numbers);
    println!("{}", grid_count);
}
