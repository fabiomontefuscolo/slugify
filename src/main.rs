use clap::Parser;

#[derive(Parser)]
struct Cli {
    strings: Vec<String>,
}

fn slugify(input: String) -> String {
    let mut result = input.to_lowercase();

    result = result.replace(' ', "-");
    result = result
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == '_' || c == '-')
        .collect();

    while result.contains("--") {
        result = result.replace("--", "-");
    }

    result = result
        .trim_matches(|c| c == ' ' || c == '-' || c == '_')
        .to_string();
    result
}

fn main() {
    let args = Cli::parse();
    let combined = args.strings.join(" ");
    let slug = slugify(combined);
    println!("{}", slug);
}
