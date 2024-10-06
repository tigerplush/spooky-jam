use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};

mod dialogue_view;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins,
        YarnSpinnerPlugin::new(),
        dialogue_view::plugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
    );
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}
