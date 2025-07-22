use clap::Parser;
use env_logger;
use log::info;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
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
    let words: Vec<String> = file.split_whitespace().map(String::from).collect();
    info!("File split");

    info!("Building lookup map");
    let mut word_map: HashMap<String, Vec<String>> = HashMap::new();
    if !words.is_empty() {
        for i in 0..(words.len() - 1) {
            word_map
                .entry(words[i].clone())
                .or_default()
                .push(words[i + 1].clone());
        }
    }
    info!("Lookup map built");

    info!("Generating sentence");
    let mut sentence: Vec<String> = vec![input.to_string()];

    // Save cursor position. We'll restore to this position on each update.
    print!("\x1B[s");
    std::io::stdout().flush().unwrap();

    for i in 0..args.length {
        let current_word = &sentence[i];
        print!("{} ", current_word.clone());

        if let Some(next_words) = word_map.get(current_word) {
            let random_indx = thread_rng().gen_range(0..next_words.len());
            let next_word = next_words[random_indx].clone();
            sentence.push(next_word.clone());
        } else {
            info!(
                "Word '{}' not in text or has no followers, stopping.",
                current_word
            );
            break;
        }

        info!("{sentence:?}");
    }
    println!(" {}", sentence.last().unwrap())
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

    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value = "8")]
    threads: usize,
}
