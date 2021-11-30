use crossbeam;
use std::{collections::HashMap, iter::repeat, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    if input.is_empty() {
        return res;
    }
    // Evenly distribute n elements amongst k threads:
    // - first n % k threads -> 1 + n / k items
    // - remaining threads -> n / k items
    let n = input.len();
    let task_count = std::cmp::min(n, worker_count);
    let chunk_size: usize = n / task_count;
    let remainder = n % task_count;
    let sizes = repeat(chunk_size + 1)
        .take(remainder)
        .chain(repeat(chunk_size).take(task_count - remainder));

    let (tx, rx) = mpsc::channel();
    crossbeam::scope(|scope| {
        let mut i = 0;
        for n in sizes {
            let tx = tx.clone();
            let chunk = &input[i..i + n];
            i += n;
            scope.spawn(move |_| {
                tx.send(count(chunk)).unwrap();
            });
        }
    })
    .unwrap();

    for _ in 0..task_count {
        let m = rx.recv().unwrap();
        for (k, v) in m.into_iter() {
            *res.entry(k).or_insert(0) += v;
        }
    }
    res
}

fn count(input: &[&str]) -> HashMap<char, usize> {
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
