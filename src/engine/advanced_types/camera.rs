use crate::engine::primitives::{matrix::Matrix4, vector::Vector3};
use std::ops::Deref;
use wgpu::util::DeviceExt;

pub struct Camera3D {
    position: Vector3<f32>,
    target: Vector3<f32>,
    // the angle between the top and bottom of the near plane relative to the position.
    fov: f32,
    // width / height of viewport
    aspect_ratio: f32,
    // z near and z far are the minimum and maximum range of the camera projection frustum
    // see https://paroj.github.io/gltut/Positioning/Tut04%20Perspective%20Projection.html
    z_near: f32, // z near can never be 0 due to the potential of division by 0 occurring.
    z_far: f32,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    camera_buffer: wgpu::Buffer,
    projection: Matrix4<f32>,
    matrix: [[f32; 4]; 4],
}

// Implement Looking at different directions
// Implement Camera position
// Cleanup
impl Camera3D {
    pub fn new(fov: f32, screen_width: u32, screen_height: u32, device: &wgpu::Device) -> Self {
        let position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let target = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let aspect_ratio = screen_width as f32 / screen_height as f32;
        let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;
        // IMPLEMENT ASPECT RATIO
        let matrix = [
            [1.0 / (aspect_ratio * (fov / 2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, 1.0 / (fov / 2.0).tan(), 0.0, 0.0],
            [0.0, 0.0, z_far / (z_far - z_near), 1.0],
            [0.0, 0.0, (-z_far * z_near) / (z_far - z_near), 0.0],
        ];
        let projection = Matrix4::new(matrix);
        let matrix = projection.to_raw();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&matrix),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera layout descriptor"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        Self {
            position,
            target,
            fov,
            aspect_ratio,
            z_near,
            z_far,
            bind_group,
            bind_group_layout,
            camera_buffer,
            matrix,
            projection,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.camera_buffer
    }

    pub fn update(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) {
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&self.matrix));
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.camera_buffer.as_entire_binding(),
            }],
        });
    }

    // column major
    pub fn create_projection_matrix(&self) -> [[f32; 4]; 4] {
        [
            [
                1.0 / (self.aspect_ratio * (self.fov / 2.0).tan()),
                0.0,
                0.0,
                0.0,
            ],
            [0.0, 1.0 / (self.fov / 2.0).tan(), 0.0, 0.0],
            [0.0, 0.0, self.z_far / (self.z_far - self.z_near), 1.0],
            [
                0.0,
                0.0,
                (-self.z_far * self.z_near) / (self.z_far - self.z_near),
                0.0,
            ],
        ]
    }
}
