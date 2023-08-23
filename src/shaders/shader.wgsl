// Vertex shader

// TODO: Implement Camera Struct

// TODO: Convert to Entity3D
struct EntityInput {
    @location(2) position: vec3<f32>,
    @location(3) rotation_one: vec3<f32>,
    @location(4) rotation_two: vec3<f32>,
    @location(5) rotation_three: vec3<f32>,
    @location(6) origin: vec3<f32>,
    @location(7) scale_one: vec3<f32>,
    @location(8) scale_two: vec3<f32>,
    @location(9) scale_three: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_pos: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>,
};


struct Camera {
    proj_mat: mat4x4<f32>,
}


@group(1) @binding(0)
var<uniform> camera_mat: mat4x4<f32>;
@vertex
fn vs_main(
    model: VertexInput,
    entity: EntityInput,
) -> VertexOutput {

    let rotation = mat3x3<f32> (
        entity.rotation_one,
        entity.rotation_two,
        entity.rotation_three,  
    );
    let scale = mat3x3<f32> (
        entity.scale_one,
        entity.scale_two,
        entity.scale_three,
    );
    // rotation is skewed and scaled..fix this. Also, use origin of object to move whole object to origin
    // not vertices as this will cause 0 multiplication to occur
    let transformed = (model.position * rotation * vec3<f32>(0.5, 0.5, 0.5)) + entity.position;
    var out: VertexOutput;
    out.tex_pos = model.tex_pos;
    out.clip_position = camera_mat * vec4<f32>(transformed, 1.0);

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
