use std::env;
use std::process;

use lib::Config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(11, part1(input));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match lib::run(config) {
        Ok(contents) => {
            part1(&contents);
        }
        Err(e) => {
            eprintln!("[ERROR] {e}");
            process::exit(1);
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut result: i32 = 0;
    let split = input.split_whitespace();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for (i, token) in split.enumerate() {
        let num = token.parse::<i32>().unwrap();

        if i % 2 == 0 {
            left.push(num);
        } else {
            right.push(num);
        }
    }


    left.sort();
    right.sort();

    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];

        result += (l - r).abs()
    }
    println!("Result: {result}");

    result
}
