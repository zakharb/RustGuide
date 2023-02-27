//Convert temperatures between Fahrenheit and Celsius.
use std::io;

fn main() {
    println!("We will calculate Celcius from Fahrenheit!");
    let line = read_line();
    let celcius: i32 = parse_string(line);
    let fahrenheit = calc_fahrenheit(celcius);
    println!("The temperature is {fahrenheit} in Fahrenheit.");

    fn calc_fahrenheit(celcius: i32) -> i32 {
        // calculate fahrehheit from celcius
        let fahrehheit = celcius * 9/5 + 32;
        return fahrehheit
    }

    fn parse_string(line: String) -> i32 {
        // parse string to integer
        let int: i32 = line.to_string().trim()
            .parse()
            .expect("Please enter degree in Celcium");
        return int
    }
    
    fn read_line() -> String {
        // read line
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        return line
    }
}
