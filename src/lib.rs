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

pub fn foo() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        foo();
    }
}
