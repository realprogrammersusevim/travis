use clap::Parser;
use env_logger;
use log::info;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::read_to_string;

fn main() {
    let args = Args::parse();

    if args.verbose {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Started");
    let input = "Hello";
    // Read "full.txt" to string
    info!("Reading file");
    let file = read_to_string(args.input).unwrap();
    info!("File read");
    info!("Splitting file");
    let words: Vec<&str> = splitter(&file);
    info!("File split");

    info!("Finding words");
    let mut sentence: Vec<String> = vec![input.to_string()];
    for i in 0..5 {
        let found = finder(&words, &sentence[i].clone());

        sentence.push(next(found));
        println!("{}", sentence.join(" "));
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10")]
    length: usize,

    #[arg(short, long, default_value = "false")]
    verbose: bool,

    #[arg(short, long, default_value = "/Volumes/Storage/git/clean.txt")]
    input: String,

    #[arg(short, long, default_value = "8")]
    threads: usize,
}

fn splitter(text: &str) -> Vec<&str> {
    // Split text by newlines and spaces
    let mut words = Vec::new();
    for line in text.split('\n') {
        for word in line.split(' ') {
            words.push(word)
        }
    }

    words
}

fn finder<'a>(words: &'a Vec<&str>, input: &str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let mut count = 0;
    for word in words {
        if word == &input {
            // Push the item after the matched word
            let found = words.get(count + 1).unwrap_or(&"");
            result.push(found);
            info!("Found word {}", found);
        }
        count += 1;
    }
    info!("Words found");
    info!("{:?}", result);

    result
}

fn next(words: Vec<&str>) -> String {
    let random_indx = thread_rng().gen_range(0..words.len());
    words[random_indx].to_string()
}
