use std::borrow::Cow;

use log::info;
use wgpu::{BlendState, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FragmentState, Instance, Limits, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PowerPreference, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, StoreOp, Surface, SurfaceConfiguration, SurfaceError, TextureViewDescriptor, VertexState};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct WGPUState {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub render_pipeline: RenderPipeline,
}

impl WGPUState {
    pub async fn new(window: Window, shader_source: &str) -> Self {
        let mut size = window.inner_size();
        size.width = size.width.max(500);
        size.height = size.height.max(500);
        info!("{:?} {:?}", window.inner_size(), size);

        let instance = Instance::default();
        info!("Created WGPU instance");

        let surface = instance.create_surface(window).unwrap();
        info!("Created WGPU surface");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        info!("Created adapter");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        info!("Created device and queue");

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);
        info!("Created config");

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
        });
        info!("Created shader");

        let render_pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        info!("Created render pipeline layout");

        let texture_format = surface.get_capabilities(&adapter).formats[0];
        let render_pipeline = device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState{
                        format: texture_format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::all(),
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview: None,
            });
        info!("Created render pipeline");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        info!("Rendering");
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder
                .begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::BLACK),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}