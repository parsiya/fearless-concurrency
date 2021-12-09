fn main() {
    let x = 5;  // let x mut = 5;
    println!("The value of x is: {}", x);
    x = 6;  // This will not work. We need to make x mutable.
    println!("The value of x is: {}", x);
}
