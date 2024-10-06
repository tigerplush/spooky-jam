use bevy::prelude::*;
use bevy_yarnspinner::{
    events::{DialogueStartEvent, PresentLineEvent, PresentOptionsEvent},
    prelude::{DialogueRunner, YarnSpinnerSystemSet},
};

use super::{
    option_selection::OptionSelection,
    setup::{UiDialogueList, UiRootNode},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_dialogue.run_if(on_event::<DialogueStartEvent>()),
            present_line.run_if(on_event::<PresentLineEvent>()),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue,
        )
            .chain()
            .after(YarnSpinnerSystemSet),
    );
}

fn show_dialogue(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Inherited;
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    query: Query<Entity, With<UiDialogueList>>,
    mut commands: Commands,
) {
    for event in line_events.read() {
        write_line(&mut commands, query.single(), event.line.character_name(), event.line.text_without_character_name().as_str());
    }
}

pub fn write_line(commands: &mut Commands, entity: Entity, speaker: Option<&str>, line: &str) {
    let mut text = String::new();
    if let Some(name) = speaker {
        text = format!("{} - ", name.to_uppercase());
    }
    text.push_str(line);
    commands
    .entity(entity)
    .with_children(|ui_dialogue_list| {
        ui_dialogue_list.spawn(TextBundle::from_section(text, TextStyle { ..default() }).with_style(style::standard()));
    });
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    option_selection: Option<Res<OptionSelection>>,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || mouse_buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if explicit_continue && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
            }
        }
    }
}

mod style {
    use bevy::prelude::*;

    pub fn standard() -> Style {
        Style {
            padding: UiRect {
                left: Val::Percent(5.0),
                right: Val::Percent(5.0),
                top: Val::Percent(1.0),
                bottom: Val::Percent(1.0),
            },
            align_self: AlignSelf::Stretch,
            ..default()
        }
    }
}