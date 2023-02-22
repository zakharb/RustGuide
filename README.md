# Content  
- [4 Ownership](#4-ownership)  
- [5 Using Structs](#5-using-structs)  
- [6 Enums and Pattern Matching](#6-enums-and-pattern-matching)  

# 4 Ownership  

## What is Ownership  
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running.

### The Stack and the Heap  
Stack > 
The stack stores values in the order it gets them and removes the values in the opposite order - `last in, first out`.
Like plates. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack.

Heap
The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location - `allocating`
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


## 6 Enums and Pattern Matching
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

## 7 Packages, Crates, and Modules
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