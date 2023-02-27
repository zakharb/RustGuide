// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
// taking advantage of the repetition in the song.
use std::io;

fn main() {

    println!("We will sing the carol 'The Twelve Days of Christmas'!");
    println!("Please input how much verses do you want (1-12)");
    let line = read_line();

    let mut number_verses: usize = parse_string(line);
    let song: Vec<String> = generate_song(number_verses);
    for line in song {
        println!("{line}");
    }

    fn generate_song(number_verses: usize) -> Vec<String> {
        let gifts = [
            "A partridge in a pear tree",
            "Two turtle doves",
            "Three French hens",
            "Four calling birds",
            "Five gold rings",
            "Six geese a-laying",
            "Seven swans a-swimming",
            "Eight maids a-milking",
            "Nine ladies dancing",
            "Ten lords a-leaping",
            "Eleven pipers piping",
            "Twelve drummers drumming",
            ];
        let mut verse: Vec<String> = Vec::new();
        let mut song: Vec<String> = Vec::new();
        for number in 1..=number_verses {
            let new_line = format!("\nOn the {number} day of Christmas my true love sent to me");
            song.push(new_line);
            let gift = gifts[number - 1];
            verse.push(gift.to_string());
            for line in &verse {
                song.push(line.to_string());
            }
        }
        return song
    }

    fn parse_string(line: String) -> usize {
        // parse string to integer
        let mut number: usize = line.to_string().trim()
            .parse()
            .expect("Please enter number of verses between 1 and 12");
        if number > 12 {
            number = 12;
        }
        return number
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
