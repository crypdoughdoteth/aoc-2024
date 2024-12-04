use day3::Parser;

pub mod day3;

fn main() {
    let part1 = {
        let parser = Parser::new("../../puzzle_input/day3.txt");
        let exprs = parser.parse();
        exprs.iter().fold(0u64, |mut acc, x| {
            acc += x.0 * x.1;
            acc
        })
    };
    let part2 = {
        let parser = Parser::new("../../puzzle_input/day3.txt");
        let exprs = parser.parse_with_flags();
        exprs.iter().fold(0u64, |mut acc, x| {
            acc += x.0 * x.1;
            acc
        })
    };
    println!("{part1}");
    println!("{part2}");
}
