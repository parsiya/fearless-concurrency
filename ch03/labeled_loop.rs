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