use clap::Parser;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Write;

fn main() {
    let args = Args::parse();

    let input = args.input;
    let file = read_to_string(args.file).unwrap();
    let words: Vec<String> = file.split_whitespace().map(String::from).collect();

    let mut word_map: HashMap<String, Vec<String>> = HashMap::new();
    if !words.is_empty() {
        for i in 0..(words.len() - 1) {
            word_map
                .entry(words[i].clone())
                .or_default()
                .push(words[i + 1].clone());
        }
    }

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
            break;
        }
    }

    println!(" {}", sentence.last().unwrap())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10")]
    length: usize,

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    file: String,
}
