#[derive(Debug)]
enum AppType {
    // name and path.
    Utility(String, String),
    // name, path, hours_played
    Game(String, String, u32),
}

fn main() {
    let calc = AppType::Utility(
        String::from("Windows Calculator"),
        String::from("C:/Windows/System32/calc.exe"),
    );

    let gw = AppType::Game(
        String::from("Guild Wars"),
        String::from("C:/Guild Wars/gw.exe"),
        5000,
    );

    println!("{:?}", calc);
    // Utility("Windows Calculator", "C:/Windows/System32/calc.exe")
    
    println!("{:?}", gw);
    // Game("Guild Wars", "C:/Guild Wars/gw.exe", 5000)
}
