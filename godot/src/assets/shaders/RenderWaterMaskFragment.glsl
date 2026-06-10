#version 450 core

layout(location = 0) flat in uint mesh_id;
layout(location = 0) out highp vec4 frag_color;


void main()
{
	// highp float g = uintBitsToFloat(1);
	// // if (floatBitsToUint(mesh_id_float) > 0) {
	// // 	g = 1.0;
	// // }
	frag_color = vec4(1 - float(gl_FrontFacing), uintBitsToFloat(mesh_id), gl_FragCoord.z, gl_FragCoord.w);
}