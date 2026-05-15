use godot::prelude::*;


struct BoundlessExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BoundlessExtension {}


pub mod framework;