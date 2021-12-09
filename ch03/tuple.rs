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