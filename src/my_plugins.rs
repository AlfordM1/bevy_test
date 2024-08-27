use crate::prelude::*;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, greet_people));

    }
}

pub fn hello_world() {
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
