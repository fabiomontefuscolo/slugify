use clap::Parser;
use slugify::slugify;

#[derive(Parser)]
struct Cli {
    strings: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let combined = args.strings.join(" ");
    let slug = slugify(combined);
    println!("{}", slug);
}
