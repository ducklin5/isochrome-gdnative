use gdnative::prelude::*;

mod game;

fn init(handle: InitHandle) {
    handle.add_class::<game::maps::Floor>();
    handle.add_class::<game::maps::FloorAgent>();
}

godot_init!(init);
