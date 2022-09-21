mod words;
use rand::prelude::*;
use words::SOHOSAISH_WORDS;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut passphrase = String::new();
    for _ in 0..6 {
        let word = SOHOSAISH_WORDS.choose(&mut rng).unwrap();
        passphrase.push_str(word);
        passphrase.push('-');
    }
    passphrase.pop();
    passphrase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let passphrase = generate();
        println!("Passphrase is {}", passphrase);
        assert!(passphrase.len() > 10);
    }
}
