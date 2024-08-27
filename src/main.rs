mod my_components;
mod my_plugins;
mod my_resources;

mod  prelude {
    pub use bevy::prelude::*;
    pub use crate::my_components::*;
    pub use crate::my_plugins::*;
    pub use crate::my_resources::*;
}
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
