#version 450 core

struct WaterInfo {
	mat4 transform;
	float water_intensity;
	float water_scale;
};

layout(location = 0) in highp vec4 world_vertex;

layout(set = 0, binding = 0) uniform sampler2D water_displacement_image; // Water Displacement Map
layout(set = 1, binding = 0) buffer InputBuffer { // Buffer of parameters for each WaterMesh
	WaterInfo water_infos[];
};

layout(push_constant, std430) uniform Params {
	restrict readonly highp mat4 world_to_clip; // World-space -> Clip-space Matrix to transform the mesh
};

layout(location = 0) flat out uint mesh_id;

void main()
{
	mesh_id = floatBitsToUint(world_vertex.w);
	WaterInfo water_info = water_infos[mesh_id];

	mat4 transform = water_info.transform;
	float water_intensity = water_info.water_intensity;
	float water_scale = water_info.water_scale;

	highp vec3 transformed_vertex = (transform * vec4(world_vertex.xyz, 1)).xyz;
	highp vec3 water_displacement = (texture(water_displacement_image, transformed_vertex.xz / water_scale).xyz * 2.0 - 1.0) * water_intensity;
	highp vec4 clip_pos = world_to_clip * vec4(transformed_vertex.xyz + water_displacement, 1.0);

	gl_Position = clip_pos;
}