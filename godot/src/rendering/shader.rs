use godot::{classes::{RdShaderSource, RenderingDevice, rendering_device::{ShaderLanguage, ShaderStage}}, obj::NewGd};
use godot::builtin::Rid;

#[must_use]
pub fn sanitize_glsl_shader_source(source: &str) -> String {
	source
		.lines()
		.filter(|line| !line.trim_start().starts_with("#["))
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn create_shader_from_source(
	rd: &mut RenderingDevice,
	source: &str,
	stage: ShaderStage,
	language: ShaderLanguage,
) -> Rid {
	let mut shader_source = RdShaderSource::new_gd();
	shader_source.set_stage_source(stage, source);
	shader_source.set_language(language);

	let Some(spirv) = rd.shader_compile_spirv_from_source(&shader_source) else {
		return Rid::Invalid;
	};

	rd.shader_create_from_spirv(&spirv)
}

pub fn create_shader_from_sources(
	rd: &mut RenderingDevice,
	sources: &[(ShaderStage, &str)],
	language: ShaderLanguage,
) -> Rid {
	let mut shader_source = RdShaderSource::new_gd();
	for (stage, source) in sources {
		shader_source.set_stage_source(*stage, *source);
	}
	shader_source.set_language(language);

	let Some(spirv) = rd.shader_compile_spirv_from_source(&shader_source) else {
		return Rid::Invalid;
	};

	rd.shader_create_from_spirv(&spirv)
}