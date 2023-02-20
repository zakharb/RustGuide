# Content
- [4.Ownership](#4-ownership)  
- [5.Using Structs](#using-structs)  

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
```
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
```
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
```
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
```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```
Instead of being called a shallow copy, it’s known as a `move`. In this example, we would say that `s1 was moved into s2`.

### Variables and Data Interacting with Clone
If we do want to `deeply copy` the heap data of the String, not just the stack data, we can use a common method called `clone`.
```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### Stack-Only Data: Copy
Integers that `have a known size at compile time` are stored entirely on the `stack`, so copies of the actual values are quick to make.
```
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
```
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
```
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
```
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
```
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
```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```
The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created.
```
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
```
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
```
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
```
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
 ```
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
```
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
```
fn first_word(s: &String) -> &str {
```
A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.
```
fn first_word(s: &str) -> &str {
```

### Other Slices
Just as we might want to refer to part of a string, we might want to refer to part of an array
```
#![allow(unused)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
}
```

### Summary
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.


# Using Structs