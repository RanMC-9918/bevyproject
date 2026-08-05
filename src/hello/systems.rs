use bevy::prelude::*;

use super::components::{CharacterName, Person};
use super::GreetTimer;

pub fn hello_world() {
    println!("hello world!");
}

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, CharacterName("Elaina Proctor".to_string())));
    commands.spawn((Person, CharacterName("Renzo Hume".to_string())));
    commands.spawn((Person, CharacterName("Zayna Nieves".to_string())));
}

pub fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&CharacterName, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn update_people(mut query: Query<&mut CharacterName, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}
