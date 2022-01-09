struct Game {
    name: String,
    hours_played: u32,
    path: String,
}

impl Game {

    fn print(&self) {
        println!("{}, {}, {}", self.name, self.hours_played, self.path);
    }

    fn calc() -> Game {
        Game {
            name: String::from("Windows Calculator"),
            hours_played: 0,
            path: String::from("C:/Windows/System32/calc.exe"),
        }
    }
}

fn main() {
    let calc = Game::calc();
    calc.print();
    // Windows Calculator, 0, C:/Windows/System32/calc.exe
}