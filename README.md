# Learning Rust

* "The book": https://doc.rust-lang.org/stable/book/
* Playground: https://play.rust-lang.org/

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
// ch02/guessing_game/src/main.rs
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

# Chapter 3
Some programming concepts in Rust.

## Mutability
We already know this. Every variable is immutable unless we use `mut`. If we
want to modify an immutable variable, the compiler warns us and does not compile
the program. See [ch03/variables/src/main.rs](ch03/variables/src/main.rs).

## Constants
Created with `const` keyword. We cannot use `mut` with them because they are
always immutable. Can only be set to a constant expression not something that is
calculated at runtime.

```rs
const FIRST_NAME = "Parsia";
const TIME: u32 = 10 * 20 * 30;
```

Naming convetion: All uppercase with underscore between words.

## Shadowing
Declare a variable with the same name. Interestingly, this can be done in
specific scopes. E.g., we can shadow a variable inside a block but it will not
be modified outside.

```rs
// ch03/shadowing/src/main.rs
fn main() {
    let x = 5;
    
    let x = x + 1; // x = 6
    // Seems like we can just create random blocks here.
    {
        let x = x * 2;  // x = 12
        // x is only shadowed in this scope.
        println!("The value of x in the inner scope is: {}", x);
    }
    // x = 6 because this one is not touched.
    println!("The value of x is: {}", x);
}
```

Seems like we can just make blocks with `{ }`.

Differences with mut:

1. We can shadow an immutable variable and create an immutable variable with the
   same name.
2. We can change the type and reuse them name. We cannot change the type of a
   mutable variable.

This works because we are shadowing `spaces` and creating a new variable of type
int.

```rs
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

We can also make the new `spaces` mutable:

```rs
// ch03/shadowing2/src/main.rs
fn main() {
    let spaces = "   ";
    println!("spaces: {}", spaces);
    let mut spaces = spaces.len();
    println!("spaces: {}", spaces);
    spaces = 1234;
    println!("spaces: {}", spaces);
}
```

returns:

```
spaces: [prints three spaces]
spaces: 3
spaces: 1234
```

If we make a variable mutable but do not modify it, the compiler will give us a
warning saying it should not be mutable.

```rs
fn main() {
    let spaces = "   ";
    println!("spaces: {}", spaces);
    let mut spaces = spaces.len();
    println!("spaces: {}", spaces);
}
```

 The shadowing `spaces` (2nd one) is mutable but not modified so we get:

```
Compiling playground v0.0.1 (/playground)
warning: variable does not need to be mutable
 --> src/main.rs:4:9
  |
4 |     let mut spaces = spaces.len();
  |         ----^^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: `playground` (bin "playground") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1.79s
     Running `target/debug/playground`
Standard Output
spaces:    
spaces: 3
```

## Integers
Like we have seen before. We can have 8, 16, 32, 64, and 128-bit integers.
Default is `i32`.

* Signed: `i8 i16 i32 i64 i128`
* Unsigned: `u8 u16 u32 u64 u128`

There's also `isize` (signed) and `usize` (unsigned) which are based on the
machine. E.g., i64 for 64-bit machines.

When writing integer literals we can use some help:

* `_` is ignored so `1_200` and `1200` are equal.
* Start
    * Hex number with `0x`: `0xAB`.
    * Octal number with `0o` (zero and the letter `o`): `0o77`.
    * Binary number with `0b`: `0b0011_1111`. See the `_` for better readability.
    * Byte with `b` (only `u8`): `b'A'`.

## Floating Point
`f32` and `f64` (default). To specify `f32` we need to annotate it:

```rs
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

## Numeric Operations
As you can imagine.

```rs
// ch03/numeric_operations.rs
fn main() {
    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum); // 5 + 10 = 15

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference); // 95.5 - 4.3 = 91.2

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product); // 4 * 30 = 120

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("56.7 / 32.2 = {}", quotient); // 56.7 / 32.2 = 1.7608695652173911
    println!("2 / 3 = {}", floored); // 2 / 3 = 0

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder); // 43 % 5 = 3
}
```

## Boolean
`true` and `false`.

## Character Type
`char`: 4-bytes Unicode Scalar values. E.g., `U+0031` or emojis.

Used with `'` (strings use `"`).

## Tuple
Fixed-size array/slice of values with different types.

```rs
// ch03/tuple.rs
fn main() {
    // create a tuple like this, note the type annotation.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructure it to get the values.
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    // x = 500, y = 6.4, z = 1

    // can also get the value by `var.index`.
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2); 
    // tup.0 = 500, tup.1 = 6.4, tup.2 = 1
}
```

## Array
Fixed-size array of values with the same type. Goes on stack instead of heap.

```rs
// ch03/arrays.rs
fn main() {
    let arr = [1, 2, 3];
    println!("arr = {:?}", arr);
    let strings = ["one", "two", "three"];
    println!("strings = {:?}", strings);

    // we can also specify the type and size. See below why we are using &str here.
    let arr2: [&str; 3] = ["one", "two", "three"];
    println!("arr2 = {:?}", arr2);
}
```

## String Literal vs. String
So I had this problem above, when you create something like this
`let a = "whatever";` you are creating a `string literal` or `&str` which is a
read-only string and not the same as the type `String`.

So when I wanted to annotate the type for the same array like this, I got an
error:

```rs
let arr2: [str, 3] = ["one", "two", "three"];
                      ^^^^^ expected `str`, found `&str`
```

So we have to annotate it like this with `&str` but I had another problem
printing it:

```rs
let arr2: [&str, 3] = ["one", "two", "three"];
println!("arr2 = {}", arr2);
                      ^^^^ `[&str; 3]` cannot be formatted with the default formatter
  = help: the trait `std::fmt::Display` is not implemented for `[&str; 3]`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

We can use the note to pretty print:

```rs
println!("arr2 = {:?}", arr2);
// arr2 = ["one", "two", "three"]

println!("arr2 = {:#?}", arr2);
// arr2 = [
//     "one",
//     "two",
//     "three",
// ]
```

Specify one initial value for all elements:

```rs
// these are the same
let a = [3; 5];
let b = [3, 3, 3, 3, 3];
```

Access array elements like most other languages:

```rs
let arr = [1, 2, 3];
let b = arr[0]; // b = 1;
```

It's possible to access elements beyond the capacity. If we the value is known
when compiling, the Rust compiler will give us an error:

```rs
fn main() {
    
    let arr2: [&str; 3] = ["one", "two", "three"];
    println!("arr2 = {:?}", arr2);
    
    println!("arr2[3] = {}", arr2[3]);
                         //  ^^^^^^^ index out of bounds: the length is 3 but the index is 3
    
    let c = 1 + 2;
    println!("arr2[c] = {}", arr2[c]);
                         //  ^^^^^^^ index out of bounds: the length is 3 but the index is 3
}
```

However, we can provide a dynamic variable (e.g., get it from the user) and the
program will panic with an out of bounds access.

## Functions
Similar to other languages.

```rs
fn main() {
    my_func(10, 'a');
}

// we can define parameters.
fn my_func(param1: i32, param2: char) {
    println!("param1 = {}, param2 = {}", param1, param2);
    // param1 = 10, param2 = a
}
```

The last expression in the function can act as the return value.

> blocks of code evaluate to the last expression in them, and numbers by
> themselves are also expressions

```rs
fn main() {
    println!("double_me(1) = {}", double_me(1)); // double_me(1) = 2
}

// return values
fn double_me(x: i32) -> i32 {
    x + 1
}
```

However, if we change it to `x + 1;` it becomes an statement and we get an
error.

```
  |
6 | fn double_me(x: i32) -> i32 {
  |    ---------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
7 |     x + 1;
  |          - help: consider removing this semicolon
```

Instead, we can use the `return` keyword.

```rs
fn main() {
    println!("double_me(1) = {}", double_me(1));
}

// return values
fn double_me(x: i32) -> i32 {
    return x + 1;
}
```

## if-else
Similar to other languages.

```rs
fn main() {
    let number = 3;

    if number < 5 {
        println!("less than five");
    } else if number == 5 {
        println!("equals five");
    } else if number > 5 {
        println!("more than five");
    }
}
```

Doing unneeded parentheses around the condition returns a warning
(e.g., `if (number < 5)`):

```rs
  |
4 |     if (number < 5) {
  |        ^          ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
4 -     if (number < 5) {
4 +     if number < 5 {
  | 
```

We can use `if` in a `let` statement:

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

## loop
We've already seen it. `loop` can use `break` and `continue` to leave to go back
to the start. When loops are nested, these only apply only to their parent loop.

We can also have `labeled loops` so we can interact with outer loops

```rs
// ch03/labeled_loop.rs
fn main() {
    let mut count = 0;
    'parent_loop: loop {
        println!("count = {} in 'parent_loop", count);

        loop {
            println!("count = {} in inside loop", count);
            if count == 2 {
                // break out of the 'parent_loop
                break 'parent_loop;
            }
            count += 1;
        }
    }
    println!("End count = {}", count);
}

// count = 0 in 'parent_loop
// count = 0 in inside loop
// count = 1 in inside loop
// count = 2 in inside loop
// End count = 2
```

We can also return values from loop (lol wut?). We can assign the loop to a
variable and then return `break value;`.

```rs
// ch03/return_value_loop.rs
fn main() {
    let mut count = 0;

    let counted = loop {
        count += 1;
        if count == 5 {
            break count;
        }
    };

    println!("counted = {}", counted); // counted = 5
}
```

## while
`loop`s don't have conditions. We can use `while` instead.

```rs
// ch03/while.rs
fn main() {
    let mut count = 0;

    while count != 5 {
        count += 1;
    }

    println!("count = {}", count); // count = 5
}
```

## Loop Through Collections with for
We can iterate through a collection (e.g., array but not tuple) with `for`.

```rs
// ch03/while.rs
fn main() {
    let strings = ["one", "two", "three"];

    for s in strings {
        println!("{}", s);
    }
}
```

Trying to use `for` with a tuple returns this error.

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);

for t in tup {
    println!("{}", t);
}

    |
10  |     for t in tup {
    |              ^^^ `(i32, f64, u8)` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `(i32, f64, u8)`
    = note: required because of the requirements on the impl of `IntoIterator` for `(i32, f64, u8)`
note: required by `into_iter`
```

The book says `for` is the most common way to use loops. We can use it to repeat
things a certain number of times by using it over a range.

```rs
// rev is reversing the range.
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// 3!
// 2!
// 1!
// LIFTOFF!!!
```



