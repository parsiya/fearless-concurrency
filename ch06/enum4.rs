struct GameStruct {
    name: String,
    path: String,
    hours_played: u32,
}

struct UtilStruct {
    name: String,
    path: String,
}

enum AppType {
    Game(GameStruct),
    Utility(UtilStruct),
}

impl AppType {
    fn stringer(&self) -> String {
        match self {
            AppType::Game(g) => format!(
                "Name: {}, Path: {}, Hours Played: {}",
                g.name, g.path, g.hours_played
            ),
            AppType::Utility(u) => format!("Name: {}, Path: {}", u.name, u.path),
        }
    }
}

fn main() {
    let calc = AppType::Utility(UtilStruct {
        name: String::from("Windows Calculator"),
        path: String::from("C:/Windows/System32/calc.exe"),
    });

    let gw = AppType::Game(GameStruct {
        name: String::from("Guild Wars"),
        path: String::from("C:/Guild Wars/gw.exe"),
        hours_played: 5000,
    });

    println!("{}", calc.stringer());
    // Name: Windows Calculator, Path: C:/Windows/System32/calc.ex

    println!("{}", gw.stringer());
    // Name: Guild Wars, Path: C:/Guild Wars/gw.exe, Hours Played: 5000
}
