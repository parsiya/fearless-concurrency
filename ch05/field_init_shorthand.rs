// define a struct
struct Game {
    name: String,
    hours_played: u32,
    path: String,
}

fn build_game(name: String, hours_played: u32, path: String) -> Game {
    Game {
        name,
        hours_played,
        path,
    }
}

fn main() {

    // create a game object
    let game1 = build_game(
        String::from("Windows Calculator"),
        123,
        String::from("C:/Windows/System32/calc.exe")
    );

    // access fields
    println!("{}, {}, {}", game1.name, game1.hours_played, game1.path);
    // Windows Calculator, 123, C:/Windows/System32/calc.exe
}