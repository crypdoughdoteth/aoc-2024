#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

pub fn is_safe(levels: &[u32]) -> bool {
    if levels[0] == levels[1] {
        return false;
    }

    let dir = if levels[0] > levels[1] {
        Direction::Decreasing
    } else {
        Direction::Increasing
    };

    levels
        .iter()
        .map_windows(|[x, y]| {
            // All increasing or decreasing => safe
            // level integer range must be 1 <= X <= 3
            if (dir == Direction::Increasing) && (x >= y || x.abs_diff(**y) > 3) {
                false
            } else if (dir == Direction::Decreasing) && (y >= x || x.abs_diff(**y) > 3) {
                false
            } else {
                true
            }
        })
        .all(bool::into)
}

pub fn day2_1() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("puzzle_input/day2.txt")?;

    let res = input.lines().fold(Vec::new(), |mut acc, e| {
        let levels = e
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        acc.push(levels);
        acc
    });

    let res = res.iter().filter(|reading| is_safe(reading)).count();

    println!("{res}");

    Ok(())
}

pub fn day2_2() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("puzzle_input/day2.txt")?;

    let res = input.lines().fold(Vec::new(), |mut acc, e| {
        let levels = e
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        acc.push(levels);
        acc
    });

    let count = res.into_iter()
        .filter(|reading| {
            is_safe(reading)
                || (0..reading.len())
                    .map(|r| {
                        let mut ls = reading.clone();
                        ls.remove(r);
                        is_safe(&ls)
                    })
                    .any(bool::into)
        })
        .count();
    println!("{count}");
    Ok(())
}
