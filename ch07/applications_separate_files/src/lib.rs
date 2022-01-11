// declare the `applications` module, content will be in `applications.rs`.
mod applications;

use applications::games::{create_guild_wars, Game};

fn create_gw() -> Game {
    // do something
    create_guild_wars()
}
