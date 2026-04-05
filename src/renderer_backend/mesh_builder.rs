use glm::*;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, vertex_attr_array};
pub struct Vertex{
    position: Vec3,
    color: Vec3,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
unsafe {    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        std::mem::size_of::<T>(),
    )
}
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer{
    let vertices: [Vertex; 3] = [
        Vertex { position: vec3(0.0, 0.5, 0.0), color: vec3(1.0, 0.0, 0.0) },
        Vertex { position: vec3(-0.5, -0.5, 0.0), color: vec3(0.0, 1.0, 0.0) },
        Vertex { position: vec3(0.5, -0.5, 0.0), color: vec3(0.0, 0.0, 1.0) },
    ];

    let bytes = unsafe { any_as_u8_slice(&vertices) };

    let buffer_descriptor = BufferInitDescriptor{
        label: Some("Vertex Buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    };
    device.create_buffer_init(&buffer_descriptor)
}