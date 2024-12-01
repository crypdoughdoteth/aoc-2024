use std::{collections::HashMap, path::Path};

pub fn day1_1() -> Result<u64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(Path::new("puzzle_input/day1.txt"))?;

    let (mut list_one, mut list_two) =
        input.lines().fold((Vec::new(), Vec::new()), |mut acc, e| {
            let ws = e.split_whitespace().collect::<Vec<&str>>();
            acc.0.push(ws[0]);
            acc.1.push(ws[1]);
            acc
        });

    list_one.sort();
    list_two.sort();

    let result = list_one
        .iter()
        .zip(list_two.iter())
        .fold(0u64, |mut acc, e| {
            let left = e.0.parse::<u64>().unwrap();
            let right = e.1.parse::<u64>().unwrap();

            let max = std::cmp::max(left, right);
            let min = std::cmp::min(left, right);
            acc += max - min;
            acc
        });
    println!("{result}");
    Ok(result)
}

pub fn day1_2() -> Result<u64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(Path::new("puzzle_input/day1.txt"))?;

    let (list_one, list_two) =
        input
            .lines()
            .fold((HashMap::new(), Vec::new()), |mut acc, e| {
                let ws = e.split_whitespace().collect::<Vec<&str>>();
                let l = ws[0].parse::<u64>().unwrap();
                let r = ws[1].parse::<u64>().unwrap();
                acc.0.insert(l, 0u64);
                acc.1.push(r);
                acc
            });

    let freqs: HashMap<u64, u64> = list_two.iter().fold(list_one, |mut acc, e| {
        if let Some(num) = acc.get(e) {
            acc.insert(*e, *num + 1);
        }
        acc
    });

    let res = freqs.iter().fold(0u64, |mut acc, e| {
        acc += e.0 * e.1;
        acc
    });

    println!("{res}");

    Ok(res)
}
