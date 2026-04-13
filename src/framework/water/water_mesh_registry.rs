use std::cell::RefCell;
use std::collections::HashSet;

use godot::builtin::{PackedInt32Array, PackedVector3Array};
use godot::classes::mesh::ArrayType;
use godot::classes::RenderingServer;
use godot::obj::{Gd, IndexEnum, InstanceId, Singleton};
use godot::prelude::*;
use godot::register::{godot_api, GodotClass};

use crate::framework::WaterMesh;

thread_local! {
	static WATER_MESH_IDS: RefCell<HashSet<InstanceId>> = RefCell::new(HashSet::new());
	static WATER_VERTICES: RefCell<PackedVector3Array> = RefCell::new(PackedVector3Array::new());
	static WATER_INDICES: RefCell<PackedInt32Array> = RefCell::new(PackedInt32Array::new());
	static REBUILD_SCHEDULED: RefCell<bool> = RefCell::new(false);
}

fn with_registry_mut<R>(mut action: impl FnMut(&mut HashSet<InstanceId>) -> R) -> R {
	WATER_MESH_IDS.with(|registry| action(&mut registry.borrow_mut()))
}

fn rebuild_buffers() {
	godot_print!("Rebuilding water mesh buffers...");
	let (vertices, indices) = collect_buffers();

	WATER_VERTICES.with(|buffer| *buffer.borrow_mut() = vertices);
	WATER_INDICES.with(|buffer| *buffer.borrow_mut() = indices);
	REBUILD_SCHEDULED.with(|scheduled| *scheduled.borrow_mut() = false);
}

fn schedule_rebuild_buffers() {
	let already_scheduled = REBUILD_SCHEDULED.with(|scheduled| {
		let mut scheduled = scheduled.borrow_mut();
		if *scheduled {
			true
		} else {
			*scheduled = true;
			false
		}
	});

	if already_scheduled {
		// godot_print!("Water mesh buffer rebuild already scheduled, skipping");
		return;
	}

	// godot_print!("Scheduling water mesh buffer rebuild");
	Callable::from_fn("rebuild_buffers", |_args| rebuild_buffers()).call_deferred(&[]);
}

fn collect_buffers() -> (PackedVector3Array, PackedInt32Array) {
	let server = RenderingServer::singleton();
	let mesh_ids = WATER_MESH_IDS.with(|registry| registry.borrow().iter().copied().collect::<Vec<_>>());

	let mut vertices = PackedVector3Array::new();
	let mut indices = PackedInt32Array::new();

	for mesh_id in mesh_ids {
		let Some(mesh_object) = Gd::<WaterMesh>::try_from_instance_id(mesh_id).ok() else {
			continue;
		};

		let mesh = mesh_object.bind();
		if !mesh.base().is_visible_in_tree() {
			continue;
		}

		let Some(surface_mesh) = mesh.get_water_mesh() else {
			continue;
		};

		let surface_count = surface_mesh.get_surface_count();
		for surface_index in 0..surface_count {
			let surface_arrays = server.mesh_surface_get_arrays(surface_mesh.get_rid(), surface_index);

			let Some(vertex_variant) = surface_arrays.get(ArrayType::VERTEX.to_index()) else {
				continue;
			};

			let Ok(surface_vertices) = vertex_variant.try_to::<PackedVector3Array>() else {
				continue;
			};

			let surface_indices = surface_arrays
				.get(ArrayType::INDEX.to_index())
				.and_then(|value| value.try_to::<PackedInt32Array>().ok())
				.unwrap_or_default();

			let vertex_offset = vertices.len() as i32;

			for vertex in surface_vertices.as_slice().iter().copied() {
				vertices.push(vertex);
			}

			for index in surface_indices.as_slice().iter().copied() {
				indices.push(index + vertex_offset);
			}
		}
	}

	(vertices, indices)
}

#[derive(GodotClass)]
#[class(init, singleton)]
pub struct WaterMeshRegistry {}

#[godot_api]
impl WaterMeshRegistry {
	pub(crate) fn mesh_ids() -> Vec<InstanceId> {
		WATER_MESH_IDS.with(|registry| {
			let mut ids = registry.borrow().iter().copied().collect::<Vec<_>>();
			ids.sort_by_key(|id| id.to_i64());
			ids
		})
	}

	pub fn add_id(id: InstanceId) {
		with_registry_mut(|registry| {
			registry.insert(id);
		});

		schedule_rebuild_buffers();
	}

	pub fn remove_id(id: InstanceId) {
		with_registry_mut(|registry| {
			registry.remove(&id);
		});

		schedule_rebuild_buffers();
	}

	pub fn schedule_rebuild() {
		schedule_rebuild_buffers();
	}

	pub fn rebuild_buffers() {
		rebuild_buffers();
	}

	pub fn gathered_vertices() -> PackedVector3Array {
		WATER_VERTICES.with(|buffer| buffer.borrow().clone())
	}

	pub fn gathered_indices() -> PackedInt32Array {
		WATER_INDICES.with(|buffer| buffer.borrow().clone())
	}

	pub fn is_empty() -> bool {
		WATER_VERTICES.with(|vertices| vertices.borrow().is_empty())
			&& WATER_INDICES.with(|indices| indices.borrow().is_empty())
	}

	pub fn clear() {
		WATER_VERTICES.with(|buffer| buffer.borrow_mut().clear());
		WATER_INDICES.with(|buffer| buffer.borrow_mut().clear());
	}

	#[func]
	pub fn add(&mut self, id: i64) {
		Self::add_id(InstanceId::from_i64(id));
	}

	#[func]
	pub fn remove(&mut self, id: i64) {
		Self::remove_id(InstanceId::from_i64(id));
	}

	#[func]
	pub fn rebuild(&mut self) {
		Self::rebuild_buffers();
	}

	#[func]
	pub fn get_gathered_vertices(&self) -> PackedVector3Array {
		Self::gathered_vertices()
	}

	#[func]
	pub fn get_gathered_indices(&self) -> PackedInt32Array {
		Self::gathered_indices()
	}

	#[func]
	pub fn has_gathered_water(&self) -> bool {
		!Self::is_empty()
	}

	#[func]
	pub fn clear_gathered_water(&mut self) {
		Self::clear();
	}
}