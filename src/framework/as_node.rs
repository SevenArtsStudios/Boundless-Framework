use godot::{classes::Node, obj::{Gd, GodotClass, Inherits, WithBaseField}};

pub trait AsNode {
	fn as_node(&self) -> Option<Gd<Node>>;
}

impl<Base, T: GodotClass> AsNode for T
	where
		Base: Inherits<Node>,
		Self: WithBaseField<Base = Base>
{
	fn as_node(&self) -> Option<Gd<Node>> {
		Some(self.base().clone().upcast())
	}
}