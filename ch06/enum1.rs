// needed to print the AppType values
#[derive(Debug)]
enum AppType {
    Utility,
    Game,
} 

struct App {
    name: String,
    hours_played: u32,
    path: String,
    app_type: AppType,
}

impl App {

    fn print(&self) {
        println!(
            "Name: {}, Hours played: {}, Path: {}, Type: {:?}",
            self.name,
            self.hours_played,
            self.path,
            self.app_type
        );
    }
}

fn main() {

    let calc = App {
        name: String::from("Windows Calculator"),
        hours_played: 123,
        path: String::from("C:/Windows/System32/calc.exe"),
        app_type: AppType::Utility,
    };

    calc.print();
    // Name: Windows Calculator, Hours played: 123, Path: C:/Windows/System32/calc.exe, Type: Utility
}