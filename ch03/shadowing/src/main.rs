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