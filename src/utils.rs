use std::io::Read;

use glsl_to_spirv::{self, ShaderType};

pub fn compile_shader(glsl: &str, shader_type: ShaderType) -> Vec<u8> {
    let mut result = Vec::new();
    let mut spirv_file = glsl_to_spirv::compile(glsl, shader_type).expect("Compilation failed");
    spirv_file.read_to_end(&mut result).unwrap();
    result
}
