use bevy::prelude::*;
use spooky_jam::plugin;

fn main() {
    App::new()
        .add_plugins(plugin)
        .run();
}
