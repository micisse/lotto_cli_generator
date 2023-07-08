use clap::Parser;

// https://docs.rs/clap/latest/clap/index.html
#[derive(Parser, Debug)]
#[command(author = "Morel Cissé", version = "1.0.0", about = None, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        // value_name = "www | common | ansible | zsh | kdbx | my | browser | vscode",
        required = true
    )]
    numbers: String,
}

fn main() {
    let default_numbers = [1..49];
    let args = Args::parse();
    let args_numbers: String = args.numbers;
    let numbers = args_numbers.split(" ").collect::<Vec<&str>>();

    println!("{:?}", numbers);
    println!("{:?}", default_numbers);
}
