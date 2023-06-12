use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rand::prelude::IteratorRandom;

fn gen_data_set() -> (HashSet<String>, HashMap<String, HashSet<String>>) {
    let f = File::open("corpus").unwrap();

    let ngram_len = 3;

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut start: HashSet<String> = HashSet::new();

    for line in BufReader::new(f).lines() {
        let line = line.unwrap();

        if !line.is_ascii() {
            continue;
        }

        if line.contains("'") {
            continue;
        }

        if line.len() < ngram_len {
            continue;
        }

        let line = line.to_lowercase() + "$";

        start.insert(line[0..ngram_len].to_string());

        for i in (ngram_len+1)..line.len()+1 {
            let key = &line[i - (ngram_len+1)..i - 1];
            let value = &line[i - 1..i];

            if !map.contains_key(key) {
                map.insert(key.to_string(), HashSet::new());
            } else {
                let set = map.get_mut(key).unwrap();
                set.insert(value.to_string());
            }
        }
    }

    (start, map)
}

fn gen_word_ngrams(start: &HashSet<String>, map: &HashMap<String, HashSet<String>>) -> String {
    loop {
        let mut word = String::new();

        let mut rng = rand::thread_rng();
        let mut key = start.iter().choose(&mut rng).unwrap().to_string();
        word.push_str(&key);

        for _ in 0..15 {
            let set = match map.get(&key) {
                Some(set) => set,
                None => break,
            };

            match set.iter().choose(&mut rng) {
                Some(value) => {
                    word.push_str(&value);
                    key = key[1..].to_string() + value;
                },
                None => break,
            }
        }

        if word.len() > 7 && word.ends_with("$") {
            word.pop();
            return word;
        }
    }
}

fn gen_word_random() -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";

    let mut rng = rand::thread_rng();
    let mut word = String::new();

    for _ in 0..10 {
        let c = chars.chars().choose(&mut rng).unwrap();
        word.push(c);
    }

    word
}

pub fn foo() {
    let (start, map) = gen_data_set();

    for _ in 0..100 {
        let ngram_word = gen_word_ngrams(&start, &map);
        let random_word = gen_word_random();

        println!("{} {}", ngram_word, random_word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        foo();
    }
}
