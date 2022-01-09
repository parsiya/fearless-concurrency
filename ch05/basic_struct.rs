// define a struct
struct Game {
    name: String,
    hours_played: u32,
    path: String,
}

fn main() {

    // create a mutable object
    let mut game1 = Game {
        name: String::from("Windows Calculator"),
        hours_played: 123,
        path: String::from("C:/Windows/System32/calc.exe"),
    };

    // access fields
    println!("{}, {}, {}", game1.name, game1.hours_played, game1.path);
    // Windows Calculator, 123, C:/Windows/System32/calc.exe

    // change fields
    game1.path = String::from("C:\\Windows\\System32\\calc.exe");
    // print the modified field
    println!("{}", game1.path);
    // C:\Windows\System32\calc.exe
}