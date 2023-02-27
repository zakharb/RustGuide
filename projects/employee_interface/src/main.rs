// Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company. For example, 
// “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people 
// in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    println!("We will add and print employees in the company!");

    loop {
        println!("Please enter 'Add employee_name to department_name'");
        println!("OR 'Get department_name' to get all employees");
        
        let line = read_line();
        let words: Vec<String> = parse_string_to_words(line);

        match words[0].as_str() {
            "Add" => {
                if words.len() > 3 {
                    let empty_department = vec![String::from("")];
                    let employee_name = words.get(1).unwrap().to_string();
                    let department_name = words.get(3).unwrap().to_string();
                    let department = departments
                        .entry(department_name)
                        .or_insert(empty_department);
                    department.push(employee_name);
                }
            }
            "Get" => {
                if words.len() > 1 {
                    let department_name = words.get(1).unwrap().to_string();
                    let employies = departments.get_mut(&department_name).unwrap();
                    employies.sort();
                    for employee in employies {
                        println!("There are emplooyies in {department_name}:");
                        println!("{employee}");
                    }
                }
            }
            _ => ()
        }

    }


    fn read_line() -> String {
        // read line
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        return line;
    }

    fn parse_string_to_words(line: String) -> Vec<String> {
        // parse string of integers to vector
        line.split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect()
    }

}
