# IO

## Read line
```rust
io::stdin().read_line(&mut line)
		   .expect("Failed to read line");
```

# Strings

## Split

### Split whitespaces
```rust
line.split_whitespace()
```

### Parse strint to vector
```rust
line.split_whitespace()
    .map(|s| s.parse().expect("parse error"))
    .collect()
```

# Vectors
## Init
```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

v.push(5);

let third: &i32 = &v[2];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);


let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

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

let len = numbers.len();

```


## Getting values
```rust
numbers[number_index]
```

## Sorting Vectors

### Sort a Vector of Integers
```rust
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
```

### Sort a Vector of Floats
```rust
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

### Sort a Vector of Structs
```rust
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}
fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // Sort people by derived natural order (Name and age)
    people.sort();
    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));
}
```

# Hash maps

## Init
```rust
let mut map = HashMap::new();
```

## Check and insert
```rust
map.entry(some_entry).or_insert(0);
```