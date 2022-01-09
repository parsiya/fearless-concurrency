// define a struct
struct Game {
    name: String,
    hours_played: u32,
    path: String,
}

fn print_game(game: &Game) {
    println!("{}, {}, {}", game.name, game.hours_played, game.path);
}

fn main() {

    // create a game object
    let game1 = Game {
        name: String::from("Windows Calculator"),
        hours_played: 123,
        path: String::from("C:/Windows/System32/calc.exe"),
    };
    
    // create game1_new based on game1 but only reuse hours_played which is the
    // only fixed-length field
    let game1_new = Game {
        name: String::from("Guild Wars"),
        path: String::from("C:/Guild Wars/gw.exe"),
        ..game1
    };
    
    // we can still use game1 here because we did not "move" any of the Strings
    // create game2 based on game1 with a new path
    print_game(&game1);
    // Guild Wars, 5000, C:/Guild Wars/gw.exe
    
    print_game(&game1_new);
    // Guild Wars, 123, C:/Guild Wars/gw.exe
    
    // create game2_new but reuse the Strings so they are "moved"
    let game1_new_new = Game {
        hours_played: 6000,
        ..game1
    };
    
    // we cannot use game1 anymore.
    print_game(&game1);  // <-- error here
    print_game(&game1_new_new);
}