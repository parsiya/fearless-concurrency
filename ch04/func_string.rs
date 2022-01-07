fn main() {
    // create a string
    let s = String::from("hello"); 

    // pass it to a function
    use_string(s);

    // we cannot use s here anymore because it was moved to `some_string` and
    // it went out of scope when `use_string` returned.
    println!("{}", s);
}

fn use_string(some_string: String) {
    println!("{}", some_string);
    // some_string goes out of scope. drop is called. Memory is freed.
}