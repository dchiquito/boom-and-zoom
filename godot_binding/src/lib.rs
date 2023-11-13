use godot::engine::{Node, NodeVirtual};
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct Player {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for Player {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self { base }
    }
}

