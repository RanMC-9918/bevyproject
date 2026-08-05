use bevy::prelude::*;

pub mod systems;
pub mod components;


#[derive(Resource)]
pub struct GreetTimer(Timer);


pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, systems::add_people);
        app.add_systems(Update, systems::hello_world);
        app.add_systems(Update, (systems::update_people, systems::greet_people).chain());
    }
}