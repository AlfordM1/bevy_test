use crate::prelude::*;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);

    }
}

// pub fn hello_world(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         println!("Hello World!");
//     }
// }
fn add_people(mut commands: Commands) {
    commands.spawn((Person, PersonName("Elaina Proctor".to_string())));
    commands.spawn((Person, PersonName("Renzo Hume".to_string())));
    commands.spawn((Person, PersonName("Zayna Nieves".to_string())));
}
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&PersonName, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
