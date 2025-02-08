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

    let mut value_array: Vec<String> = Vec::new();
    Regex::new(format!(r"{}", &args.regex).as_str())
        .unwrap()
        .find_iter(&args.input)
        .enumerate()
        .for_each(|(i, mat)| {
            value_array.push(mat.as_str().to_string());
        });
    println!("Regex Matches{:?}", value_array);

    let mut next_value = 0;
    let mut i = 0;
    let mut output = String::new();
    let char_array = args.format.chars().collect::<Vec<char>>();

    while i < char_array.len() {
        if char_array[i] == '{' && char_array[i+1] == '}' {
            output.push_str(value_array[next_value].as_str());
            next_value += 1;
            i += 2;
        } else {
            output.push(char_array[i]);
            i += 1;
        }
    }
    println!("Formatted output: {}", output);
}
// --input="test:a, thing:b" --regex="\:([a-z,A-Z,0-9]*)" --format="first:{}, second:{}"
// \:([a-z,A-Z,0-9]*)