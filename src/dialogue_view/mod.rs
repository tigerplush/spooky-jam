use bevy::prelude::*;

mod option_selection;
mod setup;
mod updating;

pub fn plugin(app: &mut App) {
    app.add_plugins(option_selection::plugin)
        .add_plugins(setup::plugin)
        .add_plugins(updating::plugin);
}
