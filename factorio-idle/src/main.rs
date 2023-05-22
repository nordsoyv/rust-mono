mod people;
mod factorio;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::factorio::FactorioPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9,0.9,1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FactorioPlugin)
        .add_startup_system(setup)
        // .add_plugin(PeoplePlugin)
        .run();
}

fn setup(commands : &mut Commands){
    commands.spawn(Camera2dBundle::default());
}