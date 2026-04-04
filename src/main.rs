use std::num::NonZero;

use wgpu::CurrentSurfaceTexture;
use winit::{event_loop::EventLoop, window::WindowAttributes};
mod renderer_backend;
use renderer_backend::pipeline_builder::PipelineBuilder;

struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    window: &'a winit::window::Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> {
    async fn new(window: &'a winit::window::Window) -> Self {
        let size = window.inner_size();
        let size = (size.width, size.height);

        let mut instance_descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        instance_descriptor.backends = wgpu::Backends::all();
        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance.create_surface(window).unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            ..Default::default()
        };

        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
        }
    }

    fn render(&mut self) {
        let drawable_enum = self.surface.get_current_texture();
        let drawable = match drawable_enum {
            CurrentSurfaceTexture::Success(frame) => frame,
            _ => {
                eprintln!("Failed to acquire next swap chain texture!");
                return;
            }
        };

        let view = drawable
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: NonZero::new(0),
        };
        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        drawable.present();
    }

    fn resize(&mut self, new_size: (u32, u32)) {
        self.size = new_size;
        self.config.width = new_size.0;
        self.config.height = new_size.1;
        self.surface.configure(&self.device, &self.config);
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = event_loop.create_window(WindowAttributes::new()).unwrap();
    let mut state = pollster::block_on(State::new(&window));

    event_loop
        .run(move |event, _elwt| match event {

            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    state.resize((physical_size.width, physical_size.height));
                }
                winit::event::WindowEvent::RedrawRequested => {
                    state.render();
                }
                winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    if event.state == winit::event::ElementState::Pressed
                        && event.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape){
                            println!("Escape key pressed, exiting...");
                            std::process::exit(0);
                        }
                }
                winit::event::WindowEvent::CloseRequested => {
                    println!("Window close requested, exiting...");
                    std::process::exit(0);
                }
                _ => {}
            },
            winit::event::Event::AboutToWait => {
                state.window.request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
