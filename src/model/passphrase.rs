mod words;
use rand::Rng;

pub fn generate() -> String {
    let mut phrases = Vec::<String>::with_capacity(5);
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let i: usize = rng.gen_range(0..words::SOHOSAISH_WORDS.len());
        let word = words::SOHOSAISH_WORDS[i];
        phrases.push(word.to_string());
    }

    phrases.join("-")
}
