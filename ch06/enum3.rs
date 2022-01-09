#[derive(Debug)]
struct GameStruct {
    name: String,
    path: String,
    hours_played: u32,
}

#[derive(Debug)]
struct UtilStruct {
    name: String,
    path: String,
}

#[derive(Debug)]
enum AppType {
    Game(GameStruct),
    Utility(UtilStruct),
}

fn main() {
    let calc = AppType::Utility(UtilStruct {
        name: String::from("Windows Calculator"),
        path: String::from("C:/Windows/System32/calc.exe"),
    });
    
    println!("{:?}", calc);
    // Utility(UtilStruct { name: "Windows Calculator", path: "C:/Windows/System32/calc.exe" })
}
