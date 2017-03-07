use std::collections::HashMap;
use std::thread;

type FrequencyMap = HashMap<char, i32>;

pub fn frequency(lines: &[&str], workers_number: usize) -> FrequencyMap {

    // Split workload
    let chunk_size = if lines.len() % workers_number == 0 && lines.len() > 0 {
        lines.len() / workers_number
    } else {
        lines.len() / workers_number + 1
    };

    let chunks: Vec<Vec<String>> = lines.chunks(chunk_size)
        .map(|chunk| chunk.iter().map(|s| s.to_string()).collect())
        .collect();

    // spawn thread and work (map)
    let mut workers = vec![];
    for chunk in chunks {
        workers.push(thread::spawn(move || {
            frequency_sequential(&chunk)
        }));
    }

    // aggregate results (reduce)
    let mut aggregated_result: FrequencyMap = HashMap::new();

    for worker in workers {
        let worker_result: FrequencyMap = worker.join().unwrap();
        for (&char, &count) in worker_result.iter() {
            *aggregated_result.entry(char).or_insert(0) += count;
        }
    }

    aggregated_result
}


pub fn frequency_sequential(lines: &[String]) -> FrequencyMap {
    let mut counts = HashMap::new();
    for line in lines {
        for c in line.to_lowercase().chars()
            .filter(|c| c.is_alphabetic()) {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    counts
}

