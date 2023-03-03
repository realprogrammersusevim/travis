use env_logger;
use log::info;
use rand::{thread_rng, Rng};
use std::fs::read_to_string;

fn main() {
    env_logger::init();

    info!("Started");
    let input = "Hello";
    // Read "full.txt" to string
    info!("Reading file");
    let file = read_to_string("/Volumes/Storage/git/clean.txt").unwrap();
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

// fn finder(words: Vec<&str>, input: &str) -> Vec<String> {
//     let mut result: Vec<String> = Vec::new();
//     let mut count = 0;
//     for word in &words {
//         if word.to_string() == input {
//             // Push the item after the matched word
//             result.push(words[count + 1].to_string());
//             info!("Found word");
//         }
//         count += 1;
//     }
//     info!("Words found");
//     info!("{:?}", result);
//
//     result
// }

fn next(words: Vec<&str>) -> String {
    let random_indx = thread_rng().gen_range(0..words.len());
    words[random_indx].to_string()
}
