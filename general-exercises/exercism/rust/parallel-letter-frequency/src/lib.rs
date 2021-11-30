use std::{collections::HashMap, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    if input.is_empty() {
        return res;
    }
    let (tx, rx) = mpsc::channel();
    let mut workers = 0;
    // TODO: Distribute work more evenly amongst threads
    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    for chunk in input.chunks(chunk_size) {
        workers += 1;
        let tx = tx.clone();
        // TODO: Avoid copying
        let chunk: Vec<String> = chunk.iter().map(|s| (*s).to_string()).collect();
        thread::spawn(move || {
            tx.send(count(&chunk)).unwrap();
        });
    }

    for _ in 0..workers {
        let m = rx.recv().unwrap();
        for (k, v) in m.into_iter() {
            *res.entry(k).or_insert(0) += v;
        }
    }
    res
}

fn count(input: &[String]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in input {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}
