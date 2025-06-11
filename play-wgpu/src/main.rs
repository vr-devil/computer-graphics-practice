use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use fundamentals_of_computer_graphics::math::Vector3;
use play_wgpu::graphics::{Vertex, WGPUInstance};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::wgt::{CommandEncoderDescriptor, TextureViewDescriptor};
use wgpu::PrimitiveTopology::{LineList, PointList};
use wgpu::{
    include_wgsl, BlendState, Buffer, BufferUsages, Color,
    ColorTargetState, ColorWrites, FragmentState, LoadOp, Operations, PipelineLayoutDescriptor,
    PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, StoreOp, SurfaceTarget,
    VertexState,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        h1 { class: "text-4xl text-center py-4", "VR-DEVIL's Graphics" }
        div {
            class: "h-svh flex place-content-center place-items-center",
            Lines {}
        }
    }
}

#[component]
pub fn Lines() -> Element {
    // let mut canvas = use_signal(|| None);

    let lines = vec![
        (Vector3::new(-0.5, 0., 0.), Vector3::new(0.5, 0., 0.)),
        (Vector3::new(0., -0.5, 0.), Vector3::new(0., 0.5, 0.)),
        (Vector3::new(-0.5, 0.5, 0.), Vector3::new(0.5, -0.5, 0.)),
        (Vector3::new(0.5, 0.5, 0.), Vector3::new(-0.5, -0.5, 0.)),
    ];

    let lines2 = lines.clone();
    use_effect(move || {
        // canvas.set(get_canvas("line-canvas"));
        for (i, &v) in lines2.iter().enumerate() {
            let id = format! {"canvas-{}", i};
            if let Some(el) = play_wgpu::get_canvas(&id) {
                let (start, end) = v;
                spawn(async move {
                    let gpu = WGPUInstance::new(SurfaceTarget::Canvas(el)).await;
                    info!("{} wgpu_instance created", id);

                    let line = LineSegment {
                        start: Vertex { position: start },
                        end: Vertex { position: end },
                    };
                    let renderer = Renderer::new(&gpu, &line);
                    renderer.render(&gpu);
                });
            } else {
                info!("failed to find canvas.")
            }
        }
    });

    rsx! {
            div {
                class: "grid grid-cols-4 gap-4",
                for (i, _) in lines.iter().enumerate() {
                    canvas { id: format!("canvas-{}", i), width: 200, height: 200 }
                }
            }

    }
}

#[derive(Copy, Clone)]
struct LineSegment {
    start: Vertex,
    end: Vertex,
}

struct Renderer {
    vertex_buffer: Buffer,
    pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(gpu: &WGPUInstance, model: &LineSegment) -> Self {
        let vertex_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buff"),
            contents: bytemuck::cast_slice(&vec![model.start, model.end]),
            usage: BufferUsages::VERTEX,
        });

        let shader = gpu
            .device
            .create_shader_module(include_wgsl!("shaders/shader.wgsl"));

        let layout = gpu
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let pipeline = gpu
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(ColorTargetState {
                        format: gpu.config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: PrimitiveState {
                    topology: LineList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });

        Self {
            vertex_buffer,
            pipeline,
        }
    }

    pub fn render(&self, gpu: &WGPUInstance) {
        let frame = gpu.surface.get_current_texture().unwrap();

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..2, 0..1);
        }

        gpu.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
