use bevy::prelude::*;

use crate::common::animation::AnimationPlugin;


pub fn init() -> App {
    let mut app = App::new();
    app.add_plugin(AnimationPlugin);
    app
}
