mod applications {
    pub mod app_enums {
        // public enum
        pub enum AppType {
            Utility,
            Game,
        }
    }
}

// absolute path
// use crate::applications::app_enums::AppType;

// can also use relative paths
// use applications::app_enums::AppType;
// or use self in the relative path
use self::applications::app_enums::AppType;

fn create_enums() {
    let calc = AppType::Utility;
    let gw = AppType::Game;
}