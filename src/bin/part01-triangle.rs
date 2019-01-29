use gfx_hal_tutorials::utils;

fn main() {
    let vertex_shader_spirv = utils::compile_shader(
        include_str!("../../assets/shaders/part01.vert"),
        glsl_to_spirv::ShaderType::Vertex,
    );

    let fragment_shader_spirv = utils::compile_shader(
        include_str!("../../assets/shaders/part01.frag"),
        glsl_to_spirv::ShaderType::Fragment,
    );
}
