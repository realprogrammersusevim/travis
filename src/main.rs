use clap::Parser;
use env_logger;
use log::info;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;

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
    let mut words = Vec::new();
    for line in file.split('\n') {
        for word in line.split(' ') {
            words.push(word)
        }
    }
    info!("File split");

    info!("Finding words");
    let mut sentence: Vec<String> = vec![input.to_string()];
    for i in 0..args.length {
        let found: &Vec<_> = &words
            .par_iter()
            .enumerate()
            .filter(|(_, word)| *word == &sentence[i].clone())
            .map(|(index, _)| words.get(index + 1).unwrap_or(&""))
            .collect();

        info!("{found:?}");

        let random_indx = thread_rng().gen_range(0..found.len());
        let next = found[random_indx].to_string();

        info!("{next:?}");

        sentence.push(next);

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

    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "/Volumes/Storage/git/clean.txt")]
    file: String,

    #[arg(short, long, default_value = "8")]
    threads: usize,
}
