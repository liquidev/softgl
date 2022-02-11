use std::time::Instant;

use bytemuck::cast_slice_mut;
use draw::TrianglePipeline;
use glam::{Mat4, Vec3, Vec4};
use pixelformat::Rgba;
use pixels::{PixelsBuilder, SurfaceTexture};
use surface::Surface;
use vertex::Viewport;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::draw::ClearPipeline;

mod draw;
mod math;
mod mesh;
mod pixelformat;
mod surface;
mod varying;
mod vertex;

#[derive(Clone, Copy)]
struct Vertex {
   position: Vec3,
   color: (f32, f32, f32),
}

impl Vertex {
   fn new(x: f32, y: f32, z: f32, color: (f32, f32, f32)) -> Self {
      Self {
         position: Vec3::new(x, y, z),
         color,
      }
   }
}

fn draw(color: &mut Surface<Rgba<u8>, &mut [Rgba<u8>]>, time: f64) {
   const RED: (f32, f32, f32) = (1.0, 0.0, 0.0);
   const GREEN: (f32, f32, f32) = (0.0, 1.0, 0.0);
   const BLUE: (f32, f32, f32) = (0.0, 0.0, 1.0);

   let mesh = [
      Vertex::new(-0.5, -0.5, 0.0, RED),
      Vertex::new(0.5, -0.5, 0.0, GREEN),
      Vertex::new(0.0, 0.5, 0.0, BLUE),
   ];

   let model = Mat4::from_rotation_z(time as f32);

   ClearPipeline {
      surface: color,
      color: Rgba {
         r: 0,
         g: 0,
         b: 0,
         a: 255,
      },
   }
   .run();

   TrianglePipeline {
      mesh: &mesh[..],
      viewport: Viewport {
         x: 0,
         y: 0,
         width: color.width(),
         height: color.height(),
      },
      color_attachment: color,
      vertex_shader: |vertex| {
         let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
         let position = model * position;
         (
            vertex::Position {
               x: position.x,
               y: position.y,
               z: position.z,
               w: position.w,
            },
            vertex.color,
         )
      },
      fragment_shader: |color| Rgba {
         r: (color.0 * 255.0) as u8,
         g: (color.1 * 255.0) as u8,
         b: (color.2 * 255.0) as u8,
         a: 255,
      },
   }
   .run();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let (width, height) = (640, 480);

   let event_loop = EventLoop::new();
   let window = WindowBuilder::new()
      .with_inner_size(LogicalSize::new(width, height))
      .with_resizable(false)
      .build(&event_loop)?;

   let surface_texture = SurfaceTexture::new(width, height, &window);
   let mut pixels = PixelsBuilder::new(width, height, surface_texture).build()?;

   let start_time = Instant::now();

   event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      match event {
         Event::WindowEvent { event, .. } => {
            if event == WindowEvent::CloseRequested {
               *control_flow = ControlFlow::Exit
            }
         }
         Event::MainEventsCleared => {
            let data = pixels.get_frame();
            let mut surface = Surface::from_buffer(width, height, cast_slice_mut(data));
            let time = start_time.elapsed().as_secs_f64();
            draw(&mut surface, time);
            if let Err(error) = pixels.render() {
               eprintln!("error: {}", error);
            }
         }
         _ => (),
      }
   });
}
