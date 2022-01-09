struct Game {
    name: String,
    hours_played: u32,
    path: String,
}

impl Game {
    // implement print for Game
    fn print(&self) {
        println!("{}, {}, {}", self.name, self.hours_played, self.path);
    }
}

fn main() {

    let game1 = Game {
        name: String::from("Guild Wars"),
        hours_played: 5000,
        path: String::from("C:/Guild Wars/gw.exe"),
    };
    
    game1.print();
    // Guild Wars, 5000, C:/Guild Wars/gw.exe
}