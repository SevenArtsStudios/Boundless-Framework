pub mod framework;

#[cfg(feature = "standalone")]
mod entry {
	use godot::prelude::*;

	struct BoundlessExtension;

	#[gdextension]
	unsafe impl ExtensionLibrary for BoundlessExtension {}
}