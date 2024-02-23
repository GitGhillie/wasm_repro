use bevy::prelude::*;
use bevy_turborand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalRng::new()) // Comment this out and wasm build will work again
        .run();
}
