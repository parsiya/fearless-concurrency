mod applications {
    pub mod games {
        // public struct
        pub struct Game {
            name: String,
            hours_played: u32,
        }
        pub fn create_guild_wars() {
            let gw = Game {
                name: String::from("Guild Wars"),
                hours_played: 5000,
            };
        }
    }
}

