use wgpu::{vertex_attr_array, BufferAddress, Device, Instance, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTarget, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::SurfaceTarget::Canvas;
use log::info;
use wgpu::wgt::DeviceDescriptor;
use bytemuck::{Pod, Zeroable};
use fundamentals_of_computer_graphics::math::Vector3;

pub struct WGPUInstance<'window> {
    pub surface: Surface<'window>,
    pub config: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
}

impl<'window> WGPUInstance<'window> {
    pub async fn new(target: SurfaceTarget<'window>) -> Self {
        let target: SurfaceTarget<'window> = target.into();

        let (width, height) = match &target {
            Canvas(canvas) => (canvas.width(), canvas.height()),
            _ => (1, 1),
        };

        let instance = Instance::default();
        info!("wgpu instance created");

        let surface = instance.create_surface(target).unwrap();
        info!("wgpu surface created");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .unwrap();
        info!("wgpu adapter created");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Device"),
                required_features: Default::default(),
                required_limits: Default::default(),
                memory_hints: Default::default(),
                trace: Default::default(),
            })
            .await
            .unwrap();
        info!("wgpu device and queue created.");

        let config = surface
            .get_default_config(&adapter, width, height)
            .unwrap();

        surface.configure(&device, &config);

        Self {
            surface,
            config,
            device,
            queue,
        }
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,
}

impl Vertex {
    const ATTRIBS: [VertexAttribute; 1] = vertex_attr_array![0 => Float32x2];

    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}