use log::info;
use wgpu::wgt::DeviceDescriptor;
use wgpu::SurfaceTarget::Canvas;
use wgpu::{
    Device, Instance, PowerPreference, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, SurfaceTarget,
};

pub struct WGPUInstance<'window> {
    pub surface: Surface<'window>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl<'window> WGPUInstance<'window> {
    pub async fn new(target: impl Into<SurfaceTarget<'window>>) -> Self {
        let target: SurfaceTarget<'window> = target.into();

        let (width, height) = match &target {
            Canvas(canvas) => (canvas.width(), canvas.height()),
            _ => (800, 800),
        };

        let instance = Instance::default();
        info!("wgpu instance created");

        let surface = instance.create_surface(target).unwrap();
        info!("wgpu surface created");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        info!("wgpu adapter created");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
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

        Self {
            surface,
            device,
            queue,
            config,
        }
    }
}
