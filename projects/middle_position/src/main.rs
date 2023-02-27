// Given a list of integers, use a vector and return the median 
// (when sorted, the value in the middle position) and mode 
// (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

fn main() {
    println!("We will calculate the median of numbers!");
    println!("Please input your numbers list like: 1 2 3 4");

    let line = read_line();
    let numbers: Vec<i32> = parse_string_to_vector(line);
    calc_mode(&numbers);
    calc_median(numbers);

    fn read_line() -> String {
        // read line
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        return line;
    }

    fn parse_string_to_vector(line: String) -> Vec<i32> {
        // parse string of integers to vector
        line.split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect()
    }

    fn calc_mode(numbers: &Vec<i32>) {
        let mut map = HashMap::new();
        let mut max = 0;
        let mut max_number: i32 = 0;
        for number in numbers {
            let count = map.entry(number).or_insert(0);
            *count += 1;
            if *count > max {
                max = *count;
                max_number = *number;
            }
        }
        println!("[+] Mode is {}", max_number);
    }

    fn calc_median(mut numbers: Vec<i32>) {
        // sort numbers, check if odd or even, calculate median
        numbers.sort();
        let len = numbers.len();
        let reminder = len % 2;
        match reminder {
            0 => calc_median_even(&numbers, len),
            1 => calc_median_odd(&numbers, len),
            _ => (),
        }
    }

    fn calc_median_even(numbers: &Vec<i32>, len: usize) {
        // calc if length numbers vector is even
        let index = len / 2;
        println!("[+] Median are {} {}", numbers[index - 1], numbers[index]);
    }

    fn calc_median_odd(numbers: &Vec<i32>, len: usize) {
        // calc if length numbers vector is odd
        let index = len / 2;
        println!("[+] Median is {}", numbers[index]);
    }
}
