fn main() {
    let mut count = 0;

    let counted = loop {
        count += 1;
        if count == 5 {
            break count;
        }
    };

    println!("counted = {}", counted);
}