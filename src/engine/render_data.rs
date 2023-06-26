use super::actors::entity::Entity2D;
use super::traits::update_textures::UpdateTextures;
use crate::engine::actors::entity::RawEntity2D;
use crate::engine::advanced_types::batch::Batch2D;
use crate::engine::primitives::vector::Vector2;
use crate::engine::primitives::vertex::{Vertex2D, Vertex3D};
use crate::engine::texture;
use crate::engine::texture::Texture2D;
use bytemuck;
use image::error::EncodingError;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use wgpu::util::BufferInitDescriptor;
use wgpu::Face::Back;
use wgpu::{util::DeviceExt, BindGroupLayout, RenderPassDescriptor, RenderPipelineDescriptor};
use winit::window::Window;

pub struct RenderData {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    window: Window,
    pipeline: wgpu::RenderPipeline,
    wall_batches: Vec<Batch2D>,
    walls: HashMap<u32, Vec<Entity2D>>,
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
        let batch_one = Batch2D::new("src/assets/yharon.png", &queue, &device, &bind_group_layout);
        let batch_two = Batch2D::new(
            "src/assets/calamitas.png",
            &queue,
            &device,
            &bind_group_layout,
        );
        let entity_one = Entity2D::new(
            batch_one.id(),
            Vector2 { x: 0, y: 0 },
            0.0,
            1.0,
            Vector2 { x: 0, y: 0 },
        );
        let entity_two = Entity2D::new(
            batch_two.id(),
            Vector2 { x: 0, y: 0 },
            0.0,
            1.0,
            Vector2 { x: 0, y: 0 },
        );

        let mut walls: HashMap<u32, Vec<Entity2D>> = HashMap::new();
        walls.insert(entity_one.texture_id(), vec![entity_one]);
        walls.insert(entity_two.texture_id(), vec![entity_two]);
        let mut wall_batches: Vec<Batch2D> = Vec::new();
        wall_batches.push(batch_one);
        wall_batches.push(batch_two);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex2D::descriptor(), RawEntity2D::descriptor()],
            },
            primitive: wgpu::PrimitiveState {
                // may want to change this to line list for raycaster...
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
            wall_batches,
            walls,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        use core::borrow;
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
            render_pass.set_pipeline(&self.pipeline);
            // THis won't work until the shader is fixed
            for batch in &mut self.wall_batches {
                batch.update(self.walls.get(&batch.id()).unwrap(), &self.device);
                render_pass.set_bind_group(0, batch.bind_group(), &[]);
                render_pass.set_vertex_buffer(0, batch.vertex_buffer().unwrap().slice(..));
                render_pass.set_vertex_buffer(1, batch.entity_buffer().unwrap().slice(..));
                render_pass.set_index_buffer(batch.index_buffer().unwrap().slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..(batch.entity_count() * 6), 0, 0..batch.entity_count())
            }
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&self) -> Result<(), String> {
        todo!()
    }
}
