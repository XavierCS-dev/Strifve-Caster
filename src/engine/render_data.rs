use super::actors::entity::Entity2D;
use super::actors::entity::Entity3D;
use super::actors::entity::RawEntity3D;
use super::advanced_types::camera_controller::CameraController3D;
use super::traits::update_textures::UpdateTextures;
use crate::engine::actors::entity::RawEntity2D;
use crate::engine::advanced_types::batch::Batch2D;
use crate::engine::advanced_types::camera::Camera3D;
use crate::engine::primitives::quaternion::Quaternion;
use crate::engine::primitives::vector::Vector2;
use crate::engine::primitives::vector::Vector3;
use crate::engine::primitives::vertex::{Vertex2D, Vertex3D};
use crate::engine::texture;
use crate::engine::texture::Texture2D;
use bytemuck;
use image::error::EncodingError;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use wgpu::Face::Back;
use wgpu::{util::DeviceExt, BindGroupLayout, RenderPassDescriptor, RenderPipelineDescriptor};
use winit::event::DeviceEvent;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::window::Window;

const VERTICES: &[Vertex3D] = &[
    // font face
    Vertex3D {
        position: [0.5, 0.5, -0.5],
        tex_pos: [1.0, 1.0 - 1.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, -0.5],
        tex_pos: [0.0, 1.0 - 1.0],
    },
    Vertex3D {
        position: [-0.5, -0.5, -0.5],
        tex_pos: [0.0, 1.0 - 0.0],
    },
    Vertex3D {
        position: [0.5, -0.5, -0.5],
        tex_pos: [1.0, 1.0 - 0.0],
    },
    // top face
    Vertex3D {
        position: [0.5, 0.5, 0.5],
        tex_pos: [1.0, 1.0 - 0.0],
    },
    Vertex3D {
        position: [-0.5, 0.5, 0.5],
        tex_pos: [0.0, 1.0 - 0.0],
    },
    // Left face
    Vertex3D {
        position: [-0.5, -0.5, 0.5],
        tex_pos: [1.0, 1.0 - 1.0],
    },
    // Bottom Face:
    Vertex3D {
        position: [0.5, -0.5, 0.5],
        tex_pos: [1.0, 0.0],
    },
    // Right face

    // Back face
];

const INDICES: &[u16] = &[
    0, 1, 2, 0, 2, 3, 4, 5, 1, 4, 1, 0, 1, 5, 6, 1, 6, 2, 3, 2, 6, 3, 6, 7, 4, 0, 3, 4, 3, 7, 5, 4,
    7, 5, 7, 6,
];

pub struct RenderData {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    window: Window,
    pipeline: wgpu::RenderPipeline,
    texture: Texture2D,
    vert_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    camera: Camera3D,
    camera_controller: CameraController3D,
    entity: Entity3D,
    entity_buf: wgpu::Buffer,
}

impl RenderData {
    pub async fn new(window: Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: adapter.features(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        // Multiple textures, all sharing bind group layout.....
        // each have separate bind group, bind group should be moved to Texture struct,
        // should be a function which provides a bind group layout with these parameters, or we create it here,
        // then store it in state for reuse

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let mut ran_vec = Vector3 {
            x: 0.0 as f32,
            y: 1.0 as f32,
            z: 0.0 as f32,
        };
        ran_vec.normalise();
        let rot = 0.0;
        let rotation = Quaternion::new(ran_vec, rot);

        // TODO: 3D Entity Creation
        let texture = Texture2D::new(
            "src/assets/calamitas.png",
            &queue,
            &device,
            &bind_group_layout,
        )
        .unwrap();

        let entity = Entity3D::new(
            None,
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 3.0,
            },
            1.0,
            rotation,
            Vec::from(VERTICES),
            Vec::new(),
        );

        let entity_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index buf"),
            contents: bytemuck::cast_slice(&[entity.to_raw()]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let vert_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vert_buf"),

            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buf"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });
        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = surface_capabilities.formats[0];
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: Vec::new(),
        };

        surface.configure(&device, &config);

        let camera = Camera3D::new(45.0, config.width, config.height, &device);
        let camera_controller = CameraController3D::new();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[&bind_group_layout, camera.bind_group_layout()],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                // TODO: Add RawEntity3D descriptor
                buffers: &[Vertex3D::descriptor(), RawEntity3D::descriptor()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        });
        Self {
            device,
            queue,
            config,
            size,
            surface,
            window,
            pipeline,
            texture,
            vert_buf,
            index_buf,
            camera,
            camera_controller,
            entity,
            entity_buf,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Make sure to change this to load, clearing is very expensive
                        // ensure there are no gaps in the space to avoid residual pixels
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            self.camera.update(&self.queue, &self.device);

            let mut ran_vec = Vector3 {
                x: 0.0 as f32,
                y: 1.0 as f32,
                z: 0.0 as f32,
            };
            ran_vec.normalise();
            self.entity.set_axis(ran_vec);
            self.entity.set_angle(self.entity.rotation().angle() - 0.25);
            render_pass.set_pipeline(&self.pipeline);

            self.queue.write_buffer(
                &self.entity_buf,
                0,
                bytemuck::cast_slice(&[self.entity.to_raw()]),
            );
            // TODO: Implement 3D rendering renderpass
            render_pass.set_bind_group(0, self.texture.bind_group(), &[]);
            render_pass.set_bind_group(1, self.camera.bind_group(), &[]);
            render_pass.set_vertex_buffer(0, self.vert_buf.slice(..));
            render_pass.set_vertex_buffer(1, self.entity_buf.slice(..));
            render_pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn update(&mut self) {}

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.camera_controller
                    .process_camera(delta.0 as f32 * 0.2, delta.1 as f32 * 0.2);
                self.camera
                    .look(&self.camera_controller.build_transformation());
            }
            _ => (),
        }
    }

    pub fn process_inputs(&mut self, event: &WindowEvent) {}

    pub fn window(&self) -> &Window {
        &self.window
    }
}
