#version 450 core

struct WaterParams {
	vec3 shallow_color;
	float fog_distance;
	vec3 deep_color;
	float fog_fade;
	float transparency_distance;
	float transparency_fade;
};

layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

layout(rgba32f, set = 0, binding = 0) uniform image2D color_image; // Color of the screen pre post-processing
layout(set = 1, binding = 0) uniform sampler2D depth_image; // Depth buffer of the screen
layout(rgba32f, set = 2, binding = 0) uniform restrict readonly image2D water_map_image; // Water Map that was rendered in the Render Shader
layout(set = 3, binding = 0) buffer InputBuffer { // Buffer for WaterMesh-specific shader information
	WaterParams water_params[];
};

layout(push_constant, std430) uniform Params {
	restrict readonly mat4 inv_projection_matrix;
	restrict readonly float near_plane;
	restrict readonly float far_plane;
	restrict readonly ivec2 screen_size; // x: screen width, y: screen height
};



float compute_water_fog_blend(float water_start, float water_end, WaterParams params) {
	float fog_blend = smoothstep(water_end, water_end + params.fog_distance, water_start);
	return clamp(exp(fog_blend * -params.fog_fade), 0.0, 1.0);
}

float compute_water_alpha_blend(float water_start, float water_end, WaterParams params) {
	float alpha_blend = smoothstep(water_end, water_end + params.transparency_distance, water_start);
	return clamp(exp(alpha_blend * -params.transparency_fade), 0.0, 1.0);
}

vec4 compute_water_color_and_alpha(float water_start, float water_end, WaterParams params) {
	float fog_blend = compute_water_fog_blend(water_start, water_end, params);
	float alpha_blend = compute_water_alpha_blend(water_start, water_end, params);
	alpha_blend = 1.0 - (fog_blend * alpha_blend);
	fog_blend = 1.0 - fog_blend;

	vec3 color = mix(params.shallow_color, params.deep_color, fog_blend);

	return vec4(color, alpha_blend);
}

vec3 compute_water_color(vec3 world_color, float water_start, float water_end, WaterParams params) {
	vec4 color = compute_water_color_and_alpha(water_start, water_end, params);
	vec3 final_color = mix(world_color, color.rgb, color.a);

	return final_color;
}


void main()
{
	ivec2 uv = ivec2(gl_GlobalInvocationID.xy);

	// r: underwater mask, underwater if > 0, out of water if 0
	// g: id of the water mesh (unpack as a uint using floatBitsToUint)
	// b: depth of the water (z)
	// a: W value of the water fragment (w)
	vec4 water_map = imageLoad(water_map_image, uv);
	if (water_map.r == 0) return; // Stop if the fragment is not under water


	vec2 screen_uv = vec2(uv) / screen_size;
	vec2 ndc = screen_uv * 2.0 - 1.0;



	highp float water_depth = water_map.z;
	vec4 water = inv_projection_matrix * vec4(ndc, water_depth, 1.0);
	water.xyz /= water.w;
	water_depth = water.z;

	highp float world_depth = textureLod(depth_image, screen_uv, 0.0).r;
	vec4 world = inv_projection_matrix * vec4(ndc, world_depth, 1.0);
	world.xyz /= world.w;
	world_depth = world.z;


	// if (world_depth < water_depth) return;
	// The amount of water we are looking through is either the end of the water volume (water_depth) or the closest surface (depth)
	highp float water_end = -min(-world_depth, -water_depth);

	WaterParams water_parameters = water_params[floatBitsToUint(water_map.g)];
	vec3 world_color = imageLoad(color_image, uv).rgb;

	// vec4 final_color = vec4(1.0, 0.0, 0.0, 1.0);
	// if (floatBitsToUint(water_map.g) != 0) {
	// 	final_color = vec4(0.0, 1.0, 0.0, 1.0);
	// }
	vec4 final_color = vec4(compute_water_color(world_color, near_plane, water_end, water_parameters), 1.0);



	imageStore(color_image, uv, final_color);
}