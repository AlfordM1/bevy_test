mod my_components;

mod  prelude {
    pub use bevy::prelude::*;
    pub use crate::my_components::*;
}
use prelude::*;

fn hello_world() {
    println!("Hello World!");
}
fn add_people(mut commands: Commands) {
    commands.spawn((Person, PersonName("Elaina Proctor".to_string())));
    commands.spawn((Person, PersonName("Renzo Hume".to_string())));
    commands.spawn((Person, PersonName("Zayna Nieves".to_string())));
}
fn greet_people(query: Query<&PersonName, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
