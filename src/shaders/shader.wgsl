// Vertex shader

struct EntityInput {
    @location(2) position: vec2<u32>,
    @location(3) rotation_one: vec2<f32>,
    @location(4) rotation_two: vec2<f32>,
    @location(5) scale_one: vec2<f32>,
    @location(6) scale_two: vec2<f32>,
    @location(7) origin: vec2<u32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_pos: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    entity: EntityInput,
) -> VertexOutput {
// TODO: do entity calculations on vertex
    var out: VertexOutput;
    out.tex_pos = model.tex_pos;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_pos);
}
