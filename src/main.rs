use clap::Parser;
use env_logger;
use log::info;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;
use std::io::Write;

fn main() {
    let args = Args::parse();

    if args.verbose {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Started");
    let input = args.input;
    // Read "full.txt" to string
    info!("Reading file");
    let file = read_to_string(args.file).unwrap();
    info!("File read");
    info!("Splitting file");
    let mut words: Vec<String> = Vec::new();
    for line in file.split('\n') {
        for word in line.split(' ') {
            words.push(word.replace("\r", ""))
        }
    }
    info!("File split");

    info!("Finding words");
    let placeholder = &String::from("");
    let mut sentence: Vec<String> = vec![input.to_string()];
    for i in 0..args.length {
        let found: &Vec<_> = &words
            .par_iter()
            .enumerate()
            .filter(|(_, word)| *word == &sentence[i].clone())
            .map(|(index, _)| words.get(index + 1).unwrap_or(placeholder))
            .collect();

        // info!("{found:?}");

        let random_indx = thread_rng().gen_range(0..found.len());
        let next = found[random_indx].to_string();

        info!("{next:?}");

        sentence.push(next);

        info!("{sentence:?}");

        print!("\r{}", sentence.join(" "));
        std::io::stdout().flush().unwrap();
    }
    println!()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10")]
    length: usize,

    #[arg(short, long, default_value = "false")]
    verbose: bool,

    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "/Volumes/Storage/git/clean.txt")]
    file: String,

    #[arg(short, long, default_value = "8")]
    threads: usize,
}
