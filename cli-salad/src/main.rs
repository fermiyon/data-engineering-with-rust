use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selman Karaosmanoglu <selmank@gmail.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    match create_fruit_salad(num_fruits) {
        Ok(salad) => println!("{:?}", salad),
        Err(e) => println!("Error: {}", e),
    }
}
