fn main() {
    let arr = [1, 2, 3];
    println!("arr = {:?}", arr);
    let strings = ["one", "two", "three"];
    println!("strings = {:?}", strings);

    // we can also specify the type and size. See below why we are using &str here.
    let arr2: [&str; 3] = ["one", "two", "three"];
    println!("arr2 = {:?}", arr2);
}