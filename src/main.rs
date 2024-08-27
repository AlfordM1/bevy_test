mod my_components;
mod my_plugins;

mod  prelude {
    pub use bevy::prelude::*;
    pub use crate::my_components::*;
    pub use crate::my_plugins::*;
}
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
