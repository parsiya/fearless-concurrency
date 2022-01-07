fn main() {
    // create an int
    let x = 5;

    // x does not move because i32 implements `Copy`.
    use_int(x); // 5
                
    // we can still use x here.
    println!("{}", x);  // 5
}

fn use_int(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.