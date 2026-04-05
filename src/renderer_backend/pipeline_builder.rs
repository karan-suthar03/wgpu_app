use std::env::current_dir;
use std::fs;
use std::num::NonZero;

use wgpu::PipelineCompilationOptions;

pub struct PipelineBuilder{
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            shader_filename: String::new(),
            vertex_entry: String::new(),
            fragment_entry: String::new(),
            pixel_format: wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }

    pub fn set_shader_module(&mut self, shader_filename: &str, vertex_entry: &str, fragment_entry: &str) {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }

    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let mut file_path = current_dir().unwrap();
        file_path.push("src/");
        file_path.push(self.shader_filename.as_str());

        let file_path = file_path.into_os_string().into_string().unwrap();
        let source_code = fs::read_to_string(file_path).expect("Failed to read shader file");

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        
        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(self.vertex_entry.as_str()),
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(self.fragment_entry.as_str()),
                targets: &render_targets,
                compilation_options: PipelineCompilationOptions::default(),
            }),

            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: true,
            },
            cache: None,
            multiview_mask: NonZero::new(0),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
        };
        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}