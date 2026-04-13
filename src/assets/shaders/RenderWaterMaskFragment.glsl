#version 450 core

layout(location = 0) in float mesh_id_float;
layout(location = 0) out highp vec4 frag_color;


void main()
{
	frag_color = vec4(1 - float(gl_FrontFacing), mesh_id_float, gl_FragCoord.z, gl_FragCoord.w);
}