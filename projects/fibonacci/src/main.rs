//Generate the nth Fibonacci number.
use std::io;

fn main() {
    println!("We will calculate the fibonacci number!");
    println!("Please input how much numbers to generate!");
    let line = read_line();
    let number: usize = parse_string(line);
    let fib_number = generate_fib(number);
    println!("The fibonacci number is {fib_number}");

    fn generate_fib(number: usize) -> usize {
        // generate fib sequence 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377
        // fib[i+1] = fib[i-1] + fib[i]
        let mut fib_vec: Vec<usize> = vec![0, 1];
        let mut i: usize = 1;
        while i < number {
            fib_vec.push(&fib_vec[i -1] + &fib_vec[i]);
            i += 1;
        }
        return fib_vec[number]
    }

    fn read_line() -> String {
        // read line
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        return line
    }

    fn parse_string(line: String) -> usize {
        // parse string to integer
        let int: usize = line.to_string().trim()
            .parse()
            .expect("Please enter positive number");
        return int
    }

}
