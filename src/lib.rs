use godot::prelude::*;


struct BoundlessExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BoundlessExtension {}


pub mod framework;


#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct GameData { }