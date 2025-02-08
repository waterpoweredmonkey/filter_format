use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short='i', long)]
    input: String,
    #[arg(short='r', long)]
    regex: String,
    #[arg(short='f', long)]
    format: String,
}

fn main() {
    let args = Args::parse();
    println!("args: {:#?}", args);
    println!("input: {}", args.input);
    println!("regex: {}", args.regex);
    println!("format: {}", args.format);

    let regex_matches = find_regex_matches(&args.input, &args.regex);
    println!("Regex Matches: {:?}", regex_matches);

    let formatted_output = apply_format(&args.format, &regex_matches);
    println!("Formatted output: {}", formatted_output);
}

fn find_regex_matches(input: &str, regex: &str) -> Vec<String> {
    Regex::new(regex)
        .unwrap()
        .find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn apply_format(format: &str, matches: &[String]) -> String {
    let mut output = String::new();
    let format_chars: Vec<char> = format.chars().collect();
    let mut i = 0;
    let mut next_match_index = 0;

    while i < format_chars.len() {
        if i + 1 < format_chars.len() && format_chars[i] == '{' && format_chars[i + 1] == '}' {
            if next_match_index < matches.len() {
                output.push_str(&matches[next_match_index]);
                next_match_index += 1;
            }
            i += 2;
        } else {
            output.push(format_chars[i]);
            i += 1;
        }
    }

    output
}
// --input="test:a, thing:b" --regex="\:([a-z,A-Z,0-9]*)" --format="first:{}, second:{}"
// \:([a-z,A-Z,0-9]*)