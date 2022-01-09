// define a struct
struct Game {
    name: String,
    hours_played: u32,
    path: String,
}

fn main() {

    // create a game object
    let game1 = Game {
        name: String::from("Windows Calculator"),
        hours_played: 123,
        path: String::from("C:/Windows/System32/calc.exe"),
    };
    
    // create game2 based on game1 with a new path
    let game2 = Game {
        path: String::from("C:\\Windows\\System32\\calc.exe"),
        ..game1
    };
    
    // access fields
    println!("{}, {}, {}", game2.name, game2.hours_played, game2.path);
    // Windows Calculator, 123, C:\Windows\System32\calc.exe
}