# Learning Rust
https://doc.rust-lang.org/stable/book/

# Chapter 1

```
cargo whatever
# create a package in `whatever` directory under the current path.
`whatever/src/main.rs` will have `hello world`.

cd whatever

cargo check     # check if it compiles without building

cargo building  # build the package

cargo run       # run
```

# Chapter 2
Every variable is immutable, have to make it mutable with `mut`:
`let mut something = 1;`

References passed to functions can also be mutable or not regardless of the
underlying variable. E.g., we can create an immutable reference to a mutable
variable. In the code below `read_line` needs a `mut string`, so we pass `&mut
guess` instead of just `&guess`.

`read_line` appends whatever is read from the command line to the argument. It
does not overwrite anything. Here, it doesn't matter because `guess` is empty.

```rs
use std::io;

fn main() {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

`expect`: The result of `read_line` is an enum of type `io::Result`. It can be
`Ok` or `Err`. If the value is `Err` then `expect` is called that crashes the
program and displays the message passed to it (`Failed to`). If the value is
`Ok` then `expectq will return the number of bytes read which we are not using
here. We can use it like this:

```rs
let bytes = io::stdin()
    // Read from input and append to guess.
    .read_line(&mut guess)
    .expect("Failed to read line");

println!("Read {} bytes", bytes);
```

Finally, we have the placeholder similar to `printf(%v)` in Go.

## Dependencies
Edit `Cargo.toml` and add depdencies like `rand = "0.8.3"`. Then `cargo build`.

`cargo update` ignores `Cargo.lock` and grabs the latest versions of libraries
(that fit the versions specified in `Cargo.toml`).

Now, we can use `rand` here like this:

```rs
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a "random" number between 1 and 100.
    // Alternative param to "1..101" is "1..=100".
    let secret_number = rand::thread_rng().gen_range(1..101);
    // removed
}
```

`1..101` range == `[1, 101)` == number between 1 and 100 == `1..=100`.

The VS Code Rust language server shows us the docs for methods (and does other
things like autocomplete), but it's also possible to see the crate docs with
`crate doc --open`. This will build the docs for each of the dependencies and
open them in the browser.

To do a comparison, we add `std::cmp::Ordering` which is another enum with three
values: `Less/Greater/Equal`.

```rs
use std::cmp::Ordering;

fn main() {
    // removed

    // Create a mutable string.
    let mut guess = String::new();

    let bytes = io::stdin()
        // Read from input and append to guess.
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // cmp compares two values
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    // removed
}
```

`cmp` compares two values. Usable on anything that can be compared. The result
is an enum of `Ordering`. We can use the `match` to do something based on what
was returned. In this case we have three `arms` (choices). Each arm has a
`pattern` (e.g., `Ordering::Less`) and an action `println!`.

This won't compile because `secret_number` is of type `integer` and `guess` is a
`string` so we cannot compare them.

```rs
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Instead of creating a new variable to store `guess` converted to an integer, we
can `shadow` the previous value. This allows us to reuse the variable name and
usually used in situations like this.

`guess.trim()` removes whitespace before/after the string. The string has the
new line character(s) because we pressed enter to finish it so it must be
trimmed.

`.parse()` converts a string into a number. To tell Rust which kind of number,
we annotate the variable with `let guess: u32`.

`.expect` returns the error if `parse` returns an error and the converted number
if the return value is `Ok`.

## Adding a Loop
We can do an infinite loop with

```rs
loop {
    println!("Please input your guess.");

    // removed

    println!("You guessed: {}", guess);
}
```

We can break out of it with `break`. We want to leave the loop when we guess the
number correctly so we add to the actions for the `Ordering::Equal` arm.

```rs
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

## Handling Invalid Input
Entering a non-number will crash the program. Let's handle it. Instead of
calling `expect` we `match` on the return value of `parse`.

```rs
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,     // If parse worked correctly, store the number in guess.
    Err(_) => continue, // If parse returned an error, go back to the beginning of the loop.
};
```

If `parse` worked correcly it returns an `Ok` value that contains the number.
Otherwise, it returns an `Err` with the error message. In `Err(_)` we are
cathing every message (with `_`).


<!-- ## Library Crates -->
