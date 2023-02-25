# Content  
- [1 Getting Started](#1-getting-started)  
- [2 Programming a Guessing Game](#2-programming-a-guessing-game)  
- [3 Comming Programming Concepts](#3-comming-programming-concepts)  
- [4 Ownership](#4-ownership)  
- [5 Using Structs](#5-using-structs)  
- [6 Enums and Pattern Matching](#6-enums-and-pattern-matching)  
- [7 Packages Crates and Modules](#7-packages-crates-and-modules)  
- [8 Common Collections](#8-common-collections)  

# 1 Getting Started

## Installation
Install
```sh
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Check, update, uninstall
```sh
rustc --version
rustup update
rustup self uninstall
```

## Hello, World!
It’s traditional when learning a new language to write a little program that prints the text `Hello, world!` to the screen, so we’ll do the same here!

### Creating a Project Directory
We suggest making a `projects directory` in your home directory and keeping all your projects there
```sh
mkdir ~/projects
cd ~/projects
mkdir hello_world
cd hello_world
```

### Writing and Running a Rust Program
Next, make a new source file and call it main.rs. 
```rust
fn main() {
    println!("Hello, world!");
}
```

Compile
```sh
rustc main.rs
./main
```

### Anatomy of a Rust Program
These lines define a function named `main`. The function body is wrapped in `{}`
```rust
fn main() {

}
```

> If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called `rustfmt`

- Rust style is to indent with `four spaces`, not a tab
- println! calls a `Rust macro`
- we pass string as an argument to `println!`
- we end the line with a semicolon `(;)`

### Compiling and Running Are Separate Steps
Before running a Rust program, you must compile it using `rustc`
```sh
rustc main.rs
```
## Hello, Cargo!
`Cargo` is Rust’s build system and `package manager`. 

### Creating a Project with Cargo
Let’s create a new project using Cargo
```sh
cargo new hello_cargo
cd hello_cargo
```

Filename: Cargo.toml - Cargo’s configuration file.
```tm
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### Building and Running a Cargo Project
Build project 
```sh
cargo build
```
Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
```sh
./target/debug/hello_cargo
```

We can also use cargo run to compile the code and then `run` the resultant executable `all in one` command:
```sh
cargo run
```

This command quickly `checks` your code to make sure it compiles but doesn’t produce an executable
```sh
cargo check
```

### Building for Release
When your project is finally ready for release, you can use command to compile it with optimizations.
```sh
cargo --build release
```

# 2 Programming a Guessing Game
We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## Setting Up a New Project
Go to the projects directory and make a `new project` using `Cargo`
```sh
cargo new guessing_game
cd guessing_game
```

## Processing a Guess
The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form.
```rust
use std::io; //standard library, known as std

fn main() { //the entry point
    println!("Guess the number!"); //macro that prints a string to the screen
    println!("Please input your guess."); // macro println!
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
```

### Storing Values with Variables
Next, we’ll create a variable to store the user input, like this:
```rust
    let mut guess = String::new();
```

To make a variable mutable, we add mut before the variable name:
```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String:  
- `let mut guess` will introduce a `mutable variable` named `guess`  
- equal sign `(=)` tells Rust we want to `bind` something to the variable  
- `::` syntax in the `::new` line indicates that new is an associated function of the `String` type  

### Receiving User Input
Call the `stdin` function from the `io` module, which will allow us to handle user `input`:
```rust
    io::stdin()
        .read_line(&mut guess)
```
> If we hadn’t `imported` the io library with `use std::io`; at the beginning of the program, we could still use the function by writing this function call as `std::io::stdin`.

Next, the line `.read_line(&mut guess)` calls the `read_line` method on the standard input handle to get `input` from the user

The `&` indicates that this argument is a `reference`, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

### Handling Potential Failure with Result
`One long line` is difficult to read, so it’s best to `divide` it
The next part is this method
```rust
        .expect("Failed to read line");
```
As mentioned earlier, `read_line` puts whatever the user enters into the string we pass to it, but it also returns a `Result` value - `enum`.
Result’s variants are `Ok` and `Err`.
An instance of `Result` has an `expect` method:
- if `Err` value, expect will cause the program to crash and display message  
- if `Ok` value, expect will take the return value and return just it  
> If you don’t call `expect`, the program will compile, but you’ll get a `warning`

### Printing Values with println! Placeholders
There’s only one more line to discuss in the code so far:
 ```rust
     println!("You guessed: {guess}");
```
> The `{}` set of curly brackets is a `placeholder`: think of {} as little crab pincers that hold a value in place.

Printing a variable and the result of an expression in one call to println! would look like this:
```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2); // "x = 5 and y = 12"
```
### Testing the First Part
Let’s test the first part of the guessing game.
```sh
cargo run
```

## Generating a Secret Number
Next, we need to generate a secret number that the user will try to guess.

### Using a Crate to Get More Functionality
The `rand` crate is a library crate, which contains code that is intended
```sh
[dependencies]
rand = "0.8.5"
```
> The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.

### Ensuring Reproducible Builds with the Cargo.lock File
When you `build` a project for the `first time`, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the `Cargo.lock` file. 

### Updating a Crate to Get a New Version
When you do want to update a crate, Cargo provides the `command update`, which will ignore the `Cargo.lock` file and figure out all the latest versions that fit your specifications in Cargo.toml. 
```sh
cargo update
```

### Generating a Random Number
Let’s start using rand to generate a number to guess.
```rust
use std::io;
use rand::Rng; //trait defines methods that random number generators implement

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
> Another neat feature of Cargo is that running the `cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser

We call the `rand::thread_rng` function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system. Then we call the `gen_range` method on the random number generator. This method is defined by the `Rng trait` that we brought into scope with the `use rand::Rng;` statement. 

## Comparing the Guess to the Secret Number
Now that we have user input and a random number, we can compare them. 
```rust
use rand::Rng;
use std::cmp::Ordering; 
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
First we add another use statement, bringing a type called `std::cmp::Ordering` into scope from the standard library. The Ordering type is another `enum` and has the variants `Less, Greater, and Equal`.

The `cmp` method compares two values and can be called on anything that can be compared. It takes a `reference` to whatever you want to compare with: here it’s comparing `guess` to `secret_number`. 

We use a `match` expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

> The core of the error states that there are `mismatched types`. Rust has a strong, static type system. However, it also has type inference. When we wrote `let mut guess = String::new()`, Rust was able to infer that guess should be a `String` and didn’t make us write the type.

Ultimately, we want to `convert` the `String` the program reads as input into a `real number type` so we can compare it numerically to the secret number:
```rust
    // --snip--
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!"); //convert str to u32
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```
> Rust allows us to `shadow` the previous value of guess with a new one. `Shadowing` lets us `reuse` the guess variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess`

The `parse` method on strings converts a string to `another type`. Here, we use it to convert from a string to a `number`. We need to tell Rust the exact number type we want by using `let guess: u32`.

## Allowing Multiple Guesses with Looping
The loop keyword creates an infinite loop.
```rust
    // --snip--
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");
        // --snip--
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```
### Handling Invalid Input
We switch from an expect call to a match expression to move from crashing on an error to handling the error.
```rust
        // --snip--
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        // --snip--
```

## Summary
This project was a hands-on way to introduce you to many new Rust concepts: let, match, functions, the use of external crates, and more.

# 3 Common Programming Concepts
This chapter covers concepts that appear in almost every programming language and how they work in Rust.

## Variables and Mutability
By default, variables are `immutable`
When a variable is immutable, once a value is bound to a name, you `can’t change` that value.
```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
Although variables are immutable by default, you can make them mutable by adding `mut` in front of the variable name 
```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

### Constants
Like `immutable variables`, constants are values that are bound to a name and `are not allowed to change`, but there are a few `differences` between constants and variables.
- `aren’t allowed to use mut` with constants.
- may be set only to a `constant expression`, not the result of a value that could only be computed at runtime.
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
Rust’s naming convention for constants is to use `all uppercase with underscores` between words

### Shadowing
You can declare a new variable with `the same name` as a previous variable - the first variable is `shadowed` by the second
We can shadow a variable by using the same variable’s name and repeating the use of the `let` keyword as follows
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```

Shadowing is `different` from marking a variable as `mut` because
- we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword
- we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name
```rust
    let spaces = "   "; // str
    let spaces = spaces.len(); // int
```

## Data Types
Every value in Rust is of a certain `data type`, which tells Rust what kind of data is being specified so it knows how to work with that data. 
Keep in mind that Rust is a `statically typed language`
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

### Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: 
- integers  
- floating point numbers  
- booleans  
- characters  

### Integer Types
An integer is a number without a fractional component `i8 i16 i32 i64 i128`, `u8 u16 u32 u64 u128`
the `isize` and `usize` types depend on the architecture of the computer 

### Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`
```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

### Numeric Operations
```rust
fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
}
```

### The Boolean Type
As in most other programming languages, a Boolean type in Rust has two possible values: `true` and `false`.
```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

### The Character Type
Rust’s `char` type is the language’s most primitive alphabetic type.
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.

### The Tuple Type
A `tuple` is a general way of grouping together a number of values with a variety of types into `one` compound type. 
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
```
We can also access a tuple element directly by using a period `(.)` followed by the index of the value we want to access
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```
The tuple without any values has a special name, `unit`.

### The Array Type
Unlike a tuple, every element of an array must have the `same type`. Unlike arrays in some other languages, arrays in Rust have a `fixed length`.
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5];
    let a = [3; 5];
}
```
A `vector` is a similar collection type provided by the standard library that is allowed to `grow or shrink` in size

### Accessing Array Elements
An array is a single chunk of memory of a known, `fixed` size that can be `allocated` on the `stack`. You can access elements of an array using `indexing`
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

### Invalid Array Element Access
Rust will `check` that the index you’ve specified is `less than` the array `length`. If the index is greater than or equal to the length, Rust will `panic`. 
This is an example of Rust’s memory `safety principles` in action. In many low-level languages, this kind of check is not done, and when you provide an `incorrect` index, invalid memory `can be accessed`. 

## Functions
`main` function, which is the `entry point` of many programs. You’ve also seen the `fn` keyword, which allows you to declare new functions.
Rust code uses `snake case` as the conventional `style` for function and variable names, in which all letters are lowercase and underscores separate words
```rust
fn main() {
    println!("Hello, world!");
    another_function();
}
fn another_function() {
    println!("Another function.");
}
```

### Parameters
We can define functions to have `parameters`
```rust
fn main() {
    another_function(5);
}
fn another_function(x: i32) { //The type of x is specified as i32
    println!("The value of x is: {x}");
}
```
In function signatures, you must declare the `type` of each `parameter`
When defining `multiple parameters`, separate the parameter declarations with `commas`

### Statements and Expressions
Rust is an `expression-based` language
- Statements are instructions that perform some action and do not return a value  
- Expressions evaluate to a resultant value  

Creating a variable and assigning a value to it with the let keyword is a statement
```rust
fn main() {
    let y = 6;
}
```

Statements do not return values. Therefore, you `can’t assign a let` statement to another variable, as the following code tries to do; you’ll get an `error`
```rust
fn main() {
    let x = (let y = 6);
}
```
The `let y = 6` statement does `not return` a value, so there isn’t anything for `x` to bind to
`Expressions` evaluate to a value and make up most of the `rest of the code` that you’ll write in Rust. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression


`Expressions` do not include `ending semicolons`. If you `add a semicolon` to the end of an expression, you `turn` it into a `statement`, and it will then not return a value.
```rust
{
    let x = 3;
    x + 1
}
```

### Functions with Return Values
We `don’t name` return values, but we must `declare` their `type` after an arrow `(->)`.
In Rust, the `return value` of the function is synonymous with the value of the `final expression` in the block of the body of a function.
You can return `early` from a function by using the `return`
```rust
fn five() -> i32 { // declare only return Type
    5 // no ; or return word
}
```

But if we place a `semicolon` at the end of the line containing x + 1, `changing` it from an `expression` to a `statement` - wll be error statements don’t evaluate to a value, which is expressed by (), the unit type
```rust
fn plus_one(x: i32) -> i32 {
    x + 1; // error here, return unit type - ()
}
```

## Comments
```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.

fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```

## Control Flow
The most common constructs that let you control the flow of execution of Rust code are `if` expressions and `loops`.

### if Expressions
```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```
 Unlike languages such as Ruby and JavaScript, Rust will `not automatically` try to `convert` `non-Boolean` types `to a Boolean`. You must be `explicit` and always provide if with a `Boolean as its condition`.
 ```rust
 fn main() {
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
}
``` 

### Handling Multiple Conditions with else if
Using `too many else if` expressions can `clutter` your code, so if you have more than one - use `match`

### Using if in a let Statement
Because `if` is an `expression`, we can use it on the `right side` of a `let` statement to assign the outcome to a variable
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```

## Repetition with Loops

Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one.

### Repeating Code with loop
You can place the `break` keyword within the loop to tell the program when to stop 
We also used `continue` in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

### Returning Values from Loops
You can `add the value` you want returned `after the break` expression you use to stop the loop
```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
```

### Loop Labels to Disambiguate Between Multiple Loops
You can optionally specify a loop `label` on a loop that you can then use with `break` or `continue`
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break internal loop
            }
            if count == 2 {
                break 'counting_up; // break external loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```

### Conditional Loops with while
A program will `often need` to evaluate a `condition` within a loop. While the condition is true, the loop runs.
```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

### Looping Through a Collection with for
You can use a `for` loop and execute some code for `each item` in a `collection`.
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}
```

# 4 Ownership  

## What is Ownership  
Ownership is a set of rules that govern how a Rust program `manages` memory.

### The Stack and the Heap  
The `stack` stores values in the order it gets them and removes the values in the opposite order - `last in, first out`.
Like plates. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack.

The `heap` is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location - `allocating`
Like Restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

### Ownership Rules  
- Each value in Rust has an owner  
- There can only be one owner at a time  
- When the owner goes out of scope, the value will be dropped  

### Variable Scope
```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

### The String Type
Rust has a second string type `String`  
This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.  
```rust
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
```

### Memory and Allocation
With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:  
- The memory must be requested from the memory allocator at runtime.  
- We need a way of returning this memory to the allocator when we’re done with our String.  

The memory is automatically returned `drop` once the variable that owns it goes `out of scope`. 
```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}
```

### Variables and Data Interacting with Move
To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid. Therefore, Rust doesn’t need to `free anything` when s1 goes out of scope. 
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```
Instead of being called a shallow copy, it’s known as a `move`. In this example, we would say that `s1 was moved into s2`.

### Variables and Data Interacting with Clone
If we do want to `deeply copy` the heap data of the String, not just the stack data, we can use a common method called `clone`.
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### Stack-Only Data: Copy
Integers that `have a known size at compile time` are stored entirely on the `stack`, so copies of the actual values are quick to make.
```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

Types implement the `Copy`   
- All the integer types, such as u32.  
- The Boolean type, bool, with values true and false.  
- All the floating-point types, such as f64.  
- The character type, char.  
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.  

### Ownership and Functions
The mechanics of passing a value to a function `are similar to those when assigning a value to a variable`. Passing a variable to a function will `move or copy`, just as assignment does.  
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope
Returning values can also `transfer ownership`.
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

## References and Borrowing

A `reference` is like a `pointer` in that it’s an address we can follow to access the data stored at that address.
Function that has a `reference to an object as a parameter instead` of taking ownership of the value
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`.
>The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`.  

We call the action of creating a reference `borrowing`. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. `You don’t own it`.  

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

### Mutable References
First we change `s` to be `mut`. Then we create a mutable reference with `&mut` s where we call the change function, and update the function signature to accept a mutable reference with `some_string: &mut String`. This makes it very clear that the change function will `mutate the value it borrows`.
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
> Mutable references have one big restriction: if you have a mutable reference to a value, you `can have no other references to that value` - `data races`

Rust enforces a similar rule for combining mutable and immutable references. 
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created.
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

### Dangling References
In languages with pointers, it’s easy to erroneously create a `dangling pointer` — a pointer that references a location in memory that may `have been given to someone else` — by freeing some memory while preserving a pointer to that memory.
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

The solution here is to return the `String` directly:
```rust
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

### The Rules of References  
Let’s recap what we’ve discussed about references:  
- At any given time, you can have either one mutable reference or any number of immutable references.  
- References must always be valid.  

### The Slice Type
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of `reference`, so it does not have `ownership`.

The problem: Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```
 
 ### String Slices
 A string slice is a `reference to part of a String`, and it looks like this:
```rust
 fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
	let slice = &s[..2]; //if you want to start at index 0
	let slice = &s[3..]; //if your slice includes the last byte
	let slice = &s[..]; //entire slice

}
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a `mutable reference`. Because `clear` needs to truncate the `String`, it needs to get a `mutable reference`. The `println!` after the call to clear uses the reference in word, so the `immutable reference must still be active at that point`. Rust disallows the mutable reference in clear and the immutable reference in word from `existing at the same time`, and compilation fails.
```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

### String Slices as Parameters
Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:
```rust
fn first_word(s: &String) -> &str {
```
A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.
```rust
fn first_word(s: &str) -> &str {
```

### Other Slices
Just as we might want to refer to part of a string, we might want to refer to part of an array
```rust
#![allow(unused)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
}
```

## Summary
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.


# 5 Using Structs

## Defining and Instantiating Structs

A `struct, or structure`, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. 
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {}
```

To get a specific value from a struct, we use dot notation.
```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

### Using the Field Init Shorthand

Because the parameter names and the struct field names are exactly the same in we can use the field init shorthand syntax
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances from Other Instances with Struct Update Syntax

It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using `struct update syntax`. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

### Using Tuple Structs Without Named Fields to Create Different Types
Rust also supports structs that look similar to tuples, called `tuple structs`.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-Like Structs Without Any Fields
You can also define structs that don’t have any fields! 
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Ownership of Struct Data

> In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.  

## An Example Program Using Structs
To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. We’ll start by using single variables, and then refactor the program until we’re using structs instead.
```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

### Refactoring with Tuples
In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.
```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

### Refactoring with Structs: Adding More Meaning
We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### Adding Useful Functionality with Derived Traits
Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // {:?} used for print debug info
}
```

Another way to print out a value using the `Debug` format is to use the `dbg! macro`
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

## Method Syntax
`Methods` are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. 

### Defining Methods
Let’s change the area function that has a `Rectangle` instance as a parameter and instead make an `area` method defined on the `Rectangle struct`.
To define the function within the context of `Rectangle`, we start an `impl` (implementation) block for Rectangle. Everything within this impl block will be `associated` with the Rectangle type
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
In the signature for area, we use `&self` instead of `rectangle: &Rectangle`. The `&self` is actually short for `self: &Self`. Within an `impl` block, the type Self is an `alias` for the type that the impl block is for. 

Methods like this are called `getters`, and Rust does not implement them automatically for struct fields as some other languages do.
```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

### Where’s the -> Operator?
> In C and C++, two different operators are used for calling methods: you use `.` if you’re calling a method on the object directly and `->` if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if `object` is a pointer, `object->something()` is similar to `(*object).something()`.
Rust doesn’t have an equivalent to the `-> operator;` instead, Rust has a feature called `automatic referencing and dereferencing`. Calling methods is one of the few places in Rust that has this behavior.

### Methods with More Parameters
Let’s practice using methods by implementing a second method on the `Rectangle` struct. This time we want an instance of Rectangle to take another instance of Rectangle and return `true` if the second Rectangle can fit completely within `self` (the first Rectangle); otherwise, it should return `false`.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### Associated Functions
All functions defined within an `impl` block are called `associated functions because` they’re associated with the type named after the impl. We can define associated functions that `don’t have self` as their first parameter (and thus are not methods) because they `don’t need an instance` of the type to work with. We’ve already used one function like this: the `String::from` function that’s defined on the String type.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3); // call this associated function
}
```

### Multiple impl Blocks
Each struct is allowed to have `multiple impl` blocks. 
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Summary
Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.


# 6 Enums and Pattern Matching
In this chapter, we’ll look at `enumerations`, also referred to as `enums`. Enums allow you to define a type by enumerating its possible `variants`. 

### Defining an Enum
Where structs give you a way of grouping together related fields and data, like a `Rectangle` with its width and height, `enums` give you a way of saying a value is one of a `possible set of values`. For example, we may want to say that `Rectangle` is one of a set of possible shapes that also includes `Circle` and `Triangle`

### Enum Values
We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, `V4` and `V6`.
```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4; // We can create instances of each 
    let six = IpAddrKind::V6;  // of the two variants of `IpAddrKind`

    route(IpAddrKind::V4); // we can call this function 
    route(IpAddrKind::V6); // with either variant: V4, V6
}

fn route(ip_kind: IpAddrKind) {} // for instance we can define a function that takes any IpAddrKind
```

Representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
```rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1")); // one liners to difine
    let loopback = IpAddr::V6(String::from("::1"));   // home and loopback
}
```

There’s another advantage to using an enum rather than a struct: `each variant can have different types and amounts of associated data`. Version four IP addresses will always have `four numeric components` that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we `wouldn’t be able to with a struct`.
```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

Let’s look at how the `standard library` defines `IpAddr`: it has the exact `enum` and `variants` that we’ve defined and used, but it embeds the address data inside the variants in the form of `two different structs`
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Let’s look at another example of an enum: this one has a wide `variety of types` embedded in its variants.
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

There is one more similarity between enums and structs: just as we’re able to define methods on structs using `impl`, we’re also able to define `methods` on enums. Here’s a method named `call` that we could define on our Message enum:
```rust
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### The Option Enum and Its Advantages Over Null Values
This section explores a case study of `Option`, which is another `enum` defined by the standard library. The Option type encodes the very common scenario in which a value could be `something` or it could be `nothing`.
Rust doesn’t have the `null` feature that many other languages have.
```rust
enum Option<T> {
    None,
    Some(T),
}
```

`<T>` means that the `Some` variant of the `Option` enum can hold `one` piece of data of any type
```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

`Option<T>` and `T` (where T can be any type) are `different types`, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

## The match Control Flow Construct
Rust has an extremely powerful control flow construct called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

Function that takes an unknown US coin and, in a similar way as the counting machine, determines which coin it is and returns its value in cents
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

If you want to run `multiple lines` of code in a match arm, you must use curly brackets, and the comma following the arm is then optional
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns That Bind to Values
Another useful feature of `match` arms is that they can bind to the `parts of the values` that match the pattern. In the match expression for this code, we add a variable called `state` to the pattern that matches values of the variant `Coin::Quarter`
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); //{:?} debug
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

### Matching with Option<T>
We can also handle `Option<T>` using `match`
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

### Matches Are Exhaustive
There’s one other aspect of match we need to discuss: the `arms patterns` must cover `all possibilities`. This will `NOT` work
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```

### Catch-all Patterns and the _ Placeholder
Using enums, we can also take special actions for a few particular values, but for `all other` values take one `default action`.
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(), // if 3 add fancy
        7 => remove_fancy_hat(), // if 7 remove fancy
        other => move_player(other), // default for all other 
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```

Let’s change the rules of the game: now, if you roll `anything other than a 3 or a 7`, you must `roll again`. We no longer need to use the catch-all value, so we can change our code to use `_` instead of the variable named `other`
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // reroll if not 7 or 3
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

Finally, we’ll change the rules of the game one more time so that `nothing` else happens on your turn `if you roll anything` other than a 3 or a 7.
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // do nothing if not 3 or 7
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```

### Concise Control Flow with if let
The `if let` syntax lets you combine if and let into a `less verbose` way to handle values that match `one` pattern while ignoring the rest.
```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
```

`Instead`, we could write this in a shorter way using `if let`.
```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

If we wanted to `count` all `non-quarter coins` we see while also announcing the state of the quarters, we could do that with a `match` expression, like this
```rust
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
```

Or we could use an `if let` and `else` expression
```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```

## Summary
We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard library’s `Option<T>` type helps you use the type system to prevent errors. When enum values have data inside them, you can use `match` or `if let` to extract and use those values, depending on how many cases you need to handle.

# 7 Packages Crates and Modules
Rust has a number of `features` that allow you to `manage` your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the `module system`, include:

- `Packages`: A Cargo feature that lets you build, test, and share crates
- `Crates`: A tree of modules that produces a library or executable
- `Modules and use`: Let you control the organization, scope, and privacy of paths
- `Paths`: A way of naming an item, such as a struct, function, or module

### Packages and Crates
A `crate` is the smallest amount of code that the Rust compiler `considers at a time`.
A crate can come in one of two forms: a `binary crate` or a `library crate`.
`Binary crates` are programs you can compile to an `executable` that you can run, such as a command-line program or a server - have function `main`
`Library crates` don’t have `main` and define functionality intended to be shared with multiple projects
```rust
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

## Defining Modules to Control Scope and Privacy

### Modules Cheat Sheet

- `Start from the crate root`: When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.

- `Declaring modules`: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:  
-- Inline, within curly brackets that replace the semicolon following `mod garden`  
-- In the file `src/garden.rs`  
-- In the file `src/garden/mod.rs`  

- `Declaring submodules`: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:  
-- Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon  
-- In the file `src/garden/vegetables.rs`  
-- In the file `src/garden/vegetables/mod.rs`  

- `Paths to code in modules`: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

- `Private vs public`: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use pub before their declarations.

- The `use` keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with use `crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

Here we create a binary crate named `backyard` that illustrates these rules. The crate’s directory, also named `backyard`, contains these files and directories:
```rust
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case is `src/main.rs`, and it contains:
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in `src/garden.rs`, which is:
```rust
pub mod vegetables;
```
Here, `pub mod vegetables;` means the code in `src/garden/vegetables.rs` is included too. That code is:
```rust
#[derive(Debug)]
pub struct Asparagus {}
```

### Grouping Related Code in Modules
`Modules` let us organize code within a crate for readability and easy reuse. Modules also allow us to control the `privacy` of items, because code within a module is `private by default`. 

In the `restaurant industry`, some parts of a restaurant are referred to as `front of house` and others as `back of house`. Front of house is where customers are; this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

Create a `new library` named restaurant by running `cargo new restaurant --lib`.
Filename: `src/lib.rs`:
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
We define a module with the `mod keyword` followed by the name of the module (in this case, `front_of_house`).

`src/main.rs` and `src/lib.rs` are called `crate roots`. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the `module tree`.
```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

```
The module tree might remind you of the `filesystem’s directory tree` on your computer; this is a very `apt` comparison!

### Paths for Referring to an Item in the Module Tree
A path can take two forms:

- An `absolute path` is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A `relative path` starts from the current module and uses `self`, `super`, or an identifier in the current module.
> Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.

Both absolute and relative paths are followed by one or more identifiers separated by double colons `(::)`.
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
> We have the correct `paths` for the hosting `module` and the `add_to_waitlist` function, but Rust won’t let us use them because it `doesn’t have access` to the private sections. 

### Exposing Paths with the pub Keyword
We want the `eat_at_restaurant` function in the parent module to have access to the `add_to_waitlist` function in the child module, so we mark the hosting module with the `pub` keyword
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
> The `pub` keyword on a module only lets code in its `ancestor modules` refer to it, not access its `inner code`

Let’s also make the `add_to_waitlist` function `public` by adding the `pub` keyword before its definition
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
> The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!

### Starting Relative Paths with super
We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path.
```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public
We can also use `pub` to designate `structs` and `enums` as `public`, but there are a few details extra to the usage of pub with structs and enums. 
We’ve defined a `public` `back_of_house::Breakfast` struct with a `public` `toast` field but a `private` `seasonal_fruit` field
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In contrast, if we make an `enum` `public`, all of its variants are then `public`. We only need the `pub` before the enum keyword
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### Bringing Paths into Scope with the use Keyword
We bring the `crate::front_of_house::hosting` module into the scope of the `eat_at_restaurant` function so we only have to specify `hosting::add_to_waitlist` to call the `add_to_waitlist` function in eat_at_restaurant
** Note that use only creates the shortcut for the `particular scope` in which the `use` occurs.**
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
> Adding `use` and a `path` in a scope is similar to creating a `symbolic link` in the filesystem.

### Creating Idiomatic use Paths
Specifying the `parent module` when calling the function makes it clear that the function isn’t `locally defined` while still minimizing repetition of the full path. 

On the other hand, when bringing in `structs, enums, and other items` with `use`, it’s idiomatic to specify the `full path`.
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
> The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.

### Providing New Names with the as Keyword
There’s another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or alias, for the type. 
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with pub use
We can combine `pub` and `use`. This technique is called `re-exporting` because we’re bringing an item into scope but also making that item available for others to bring into their scope.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### Using External Packages
To bring `rand` definitions into the scope of our package, we added a use line starting with the name of the crate
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

with `HashMap` we would use this line:
```rust
use std::collections::HashMap;
```

### Using Nested Paths to Clean Up Large use Lists
We can use nested paths to bring items into scope `in one line`
```rust
use std::{cmp::Ordering, io};
use std::io::{self, Write};
```

### The Glob Operator
If we want to bring `all` public items
```rust
use std::collections::*;
```

## Separating Modules into Different Files
We’ll extract `modules` into `files` instead of having all the modules defined in the crate root file. 

Filename: src/lib.rs
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Filename: src/front_of_house.rs
```rust
pub mod hosting;
```

Filename: src/front_of_house/hosting.rs
```rust
pub fn add_to_waitlist() {}
```

## Summary
Rust lets you split a `package` into multiple `crates` and a crate into `modules` so you can refer to items defined in one module from another module. You can do this by specifying `absolute` or `relative` paths. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope. Module code is `private by default`, but you can make definitions public by adding the `pub` keyword.


# 8 Common Collections
Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.  
- A `vector` allows you to store a variable number of values next to each other.
- A `string` is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A `hash map` allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## Storing Lists of Values with Vectors
The first collection type we’ll look at is `Vec<T>`, also known as a `vector`.
To create a new empty vector, we call the `Vec::new` function,
```rust
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
```

### Updating a Vector
To create a vector and then add elements to it, we can use the `push` method,
```rust
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
```

### Reading Elements of Vectors
There are `two` ways `to reference` a value stored in a vector: via `indexing` or using the `get`
```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // using index
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // using get
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```
`Indexing` - this method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
When the `get` method is passed an index that is outside the vector, it returns None `without panicking`.

### Iterating over the Values in a Vector
To `access each element` in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time with `for loop`
```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

We can also `iterate` over `mutable` references
```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

```

### Using an Enum to Store Multiple Types
When we need `one type` to represent `elements of different types`, we can define and use an `enum`
```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

### Dropping a Vector Drops Its Elements
Like any other struct, a vector is freed when it goes out of scope
```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

## Storing UTF-8 Encoded Text with Strings
New Rustaceans commonly get stuck on strings for a combination of three reasons: 
- Rust’s propensity for exposing possible errors  
- strings being a more complicated data structure than many programmers give them credit for  
- and UTF-8  

### What Is a String?
Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`
The `String` type, which is provided by Rust’s `standard library` rather than coded into the core language, is a `growable`, `mutable`, `owned`, `UTF-8` encoded string type. 

### Creating a New String
Many of the `same operations` available with `Vec<T>` are available with `String` as well, because String is actually implemented as a `wrapper around a vector` of bytes with some extra guarantees, restrictions, and capabilities.
```rust
    let mut s = String::new();
```

Often, we’ll have some `initial data` that we want to start the string with. For that, we `use` the `to_string` method, which is available on any type that implements the `Display` trait
```rust
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
```

We `can also` use the function `String::from` to create a String from a string literal.
```rust
    let s = String::from("initial contents");
```
> In this case, `String::from` and `to_string` do the `same` thing, so which you choose is a matter of style and readability.

### Updating a String
A String `can grow` in size and its contents can change, just like the contents of a `Vec<T>`, if you `push` more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate String values.

We can `grow` a String by using the `push_str` method to
```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```
> The push_str method takes a string slice and `don’t take ownership` of the parameter

The `push` method takes a `single character` as a parameter and adds it to the String.
```rust
    let mut s = String::from("lo");
    s.push('l');
```

Often, you’ll want to `combine two` existing strings. One way to do so is to `use` the `+` operator
```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
> The string s3 will contain Hello, world!. The reason `s1` is `no longer valid` after the addition, and the reason we used a `reference to s2`, has to do with the signature of the `method` that’s called when we use the + operator. The `+` operator `uses` the `add` method

If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy. 
For more `complicated string` combining, we can instead `use` the `format!` macro:
```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
```

### Indexing into Strings
Rust strings `don’t support indexing`
A `String` is a wrapper over a `Vec<u8>`.
Sometimes `UTF-8` stores as `1 byte`, sometimes with Unicode scalar value as `2 bytes`.
To avoid `returning an unexpected value` and causing `bugs` that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

### Bytes and Scalar Values and Grapheme Clusters! Oh My!
Another point about `UTF-8` is that there are actually `three` relevant `ways to look` at strings from Rust’s perspective: `as bytes`, `scalar values`, and `grapheme clusters`
> A final reason Rust `doesn’t allow us to index` into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because `Rust would have to walk through the contents from the beginning to the index` to determine `how many valid` characters there were.

### Slicing Strings
`Indexing` into a string is `often a bad idea` because it’s `not clear` what the return type of the string-indexing `operation should be`: a byte value, a character, a grapheme cluster, or a string slice.

### Methods for Iterating Over Strings
The `best way` to operate on pieces of strings is to be `explicit` about whether you want `characters or bytes`. For individual Unicode scalar values, use the `chars` method.
```rust
for c in "Зд".chars() {
    println!("{c}");
}
```
Alternatively, the `bytes` method returns each `raw byte`
```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

### Strings Are Not So Simple
To summarize, `strings are complicated`. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the `correct handling of String data the default` behavior for all Rust programs, which means programmers have to put more thought into `handling UTF-8` data upfront

## Storing Keys with Associated Values in Hash Maps
The type `HashMap<K, V>` stores a mapping of `keys` of type K to `values` of type V using a hashing function, which determines how it places these keys and values into memory. 
Hash maps are `useful` when you want to look up data `not by using an index`, as you can with vectors, but `by using a key` that can be of any type. 

### Creating a New Hash Map
One way to create an empty hash map is using new and adding elements with insert. Just like vectors, hash maps store their data on the heap. 
```rust
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```
> we need to first `use the HashMap from the collections` portion of the standard library.

### Accessing Values in a Hash Map
We `can get a value` out of the hash map by providing its key to the `get` method
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```
> Here, `score` will have the value that’s associated with the `Blue` team, and the result will be `10`. The `get` method returns an `Option<&V>`; if there’s `no value` for that key in the hash map, get will return `None`. This program handles the `Option` by calling `copied` to get an `Option<i32>` rather than an `Option<&i32>`, then `unwrap_or` to set score to zero if scores doesn't have an entry for the key.

We can `iterate` over each key/value pair in a `hash map` in a similar manner as we do with vectors, using a `for loop`
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

### Hash Maps and Ownership
For types that implement the `Copy trait, like i32`, the values are `copied into the hash` map. `For owned` values like `String`, the values will be `moved` and the hash map will be the `owner` of those values
```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```
> We `aren’t able to use` the variables `field_name` and `field_value` after they’ve been moved into the hash map with the call to insert.

### Updating a Hash Map
When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned.

Overwriting a Value
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```
> If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced

Adding a Key and Value Only If a Key Isn’t Present
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```
> if the key does exist in the hash map, the existing value should remain the way it is. If the key doesn’t exist, insert it and a value for it.

Updating a Value Based on the Old Value
```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```
> We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word

### Hashing Functions
By default, HashMap uses a hashing function called `SipHash` 

