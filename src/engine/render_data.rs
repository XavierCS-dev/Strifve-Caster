use super::actors::entity::Entity2D;
use super::traits::update_textures::UpdateTextures;
use crate::engine::actors::entity::RawEntity2D;
use crate::engine::advanced_types::texture_vecs::Texture2DMap;
use crate::engine::primitives::vector::Vector2;
use crate::engine::primitives::vertex::{Vertex2D, Vertex3D};
use crate::engine::texture;
use crate::engine::texture::Texture2D;
use crate::engine::texture::TEXTURE_IDS;
use bytemuck;
use std::collections::HashMap;
use wgpu::util::BufferInitDescriptor;
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
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    textures: Texture2DMap,
    // These may be part of the texture in the future when I work out what to do with them
    bind_group_layout: wgpu::BindGroupLayout,
    /* TODO Will need to separate walls and sprites here... possibly different Hashmaps.
        eg, wall_entities, sprite entities, separate loops iterating through them, with sprites coming after1
        K: TextureID, V: Batch of entities
     */
    entities: HashMap<u32, Vec<Entity2D>>,
    entity_buffer: wgpu::Buffer,
}

// Temp, to be removed
const VERTICES: &[Vertex2D] = &[
    Vertex2D {
        position: [-0.0868241, 0.49240386],
        tex_pos: [1.0 - 0.4131759, 1.0 - 0.99240386],
    }, // A
    Vertex2D {
        position: [-0.49513406, 0.06958647],
        tex_pos: [1.0 - 0.0048659444, 1.0 - 0.56958647],
    }, // B
    Vertex2D {
        position: [-0.21918549, -0.44939706],
        tex_pos: [1.0 - 0.28081453, 1.0 - 0.05060294],
    }, // C
    Vertex2D {
        position: [0.35966998, -0.3473291],
        tex_pos: [1.0 - 0.85967, 1.0 - 0.1526709],
    }, // D
    Vertex2D {
        position: [0.44147372, 0.2347359],
        tex_pos: [1.0 - 0.9414737, 1.0 - 0.7347359],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /**/ 2, 3, 4];

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
        let tex =
            texture::Texture2D::new("src/assets/yharon.png", &queue, &device, &bind_group_layout)
                .unwrap();
        let tex_one_id = tex.id();
        let tex_two = texture::Texture2D::new(
            "src/assets/calamitas.png",
            &queue,
            &device,
            &bind_group_layout,
        )
        .unwrap();
        let tex_two_id = tex_two.id();
        let mut textures = Texture2DMap::new();
        textures.add_texture(tex);
        textures.add_texture(tex_two);
        let entity_one = Entity2D::new(
            tex_one_id,
            Vector2 { x: 0, y: 0 },
            0.0,
            1.0,
            Vector2 { x: 0, y: 0 },
        );
        let entity_two = Entity2D::new(
            tex_one_id,
            Vector2 { x: 0, y: 0 },
            0.0,
            1.0,
            Vector2 { x: 0, y: 0 },
        );

        // TEMPORARY
        let entities: HashMap<u32, Vec<Entity2D>> = HashMap::new();
        let entity_vec = entities
            .iter()
            .map(|x| x.1)
            .flatten()
            .map(|z| z.to_raw())
            .collect::<Vec<RawEntity2D>>();
        let entity_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity Buffer"),
            contents: bytemuck::cast_slice(&entity_vec),
            usage: wgpu::BufferUsages::VERTEX,
        });

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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // temp, vertices_count will be init with 0 when this is gone.
        let index_count: u32 = INDICES.len() as u32;

        Self {
            device,
            queue,
            config,
            size,
            surface,
            window,
            pipeline,
            vertex_buffer,
            index_buffer,
            index_count,
            textures,
            bind_group_layout,
            entities,
            entity_buffer,
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

            render_pass.set_pipeline(&self.pipeline);
            for batch in &self.entities {
                // TODO: keys with empty values may be an issue, this will need to be checked for and removed when textures are removed
                render_pass.set_bind_group(
                    0,
                    self.textures.inner().get(&batch.0).unwrap().bind_group(),
                    &[],
                );
                // TODO: Create Vertex Buffer

                let entities_vec: Vec<RawEntity2D> = batch.1.iter().map(|x| x.to_raw()).collect();
                self.entity_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Entities Buffer"),
                    contents: bytemuck::cast_slice(&entities_vec),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                render_pass.set_vertex_buffer(1, self.entity_buffer.slice(..));
                /* TODO, create index buffer that updates when new entities are added or removed, should be the
                    same pattern of indices each time (eg 1,2,3,4).
                 */
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.index_count, 0, 0..1);
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
