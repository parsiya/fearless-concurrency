// public struct
pub struct Game {
    name: String,
    hours_played: u32,
}
pub fn create_guild_wars() -> Game {
    Game {
        name: String::from("Guild Wars"),
        hours_played: 5000,
    }
}