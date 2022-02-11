use std::time::Instant;

use bytemuck::cast_slice_mut;
use draw::TrianglePipeline;
use glam::{Mat4, Vec3, Vec4};
use math::deg_to_rad;
use nanorand::Rng;
use pixelformat::{Depth, Rgba};
use pixels::{PixelsBuilder, SurfaceTexture};
use surface::Surface;
use vertex::Viewport;
use wavefront_obj::obj::{self, Primitive};
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
   fn new(x: f32, y: f32, z: f32) -> Self {
      let mut rng = nanorand::tls_rng();
      Self {
         position: Vec3::new(x, y, z),
         color: (
            rng.generate_range(0..255) as f32 / 255.0,
            rng.generate_range(0..255) as f32 / 255.0,
            rng.generate_range(0..255) as f32 / 255.0,
         ),
      }
   }
}

struct State {
   depth: Surface<Depth<f32>>,
   model: (Vec<Vertex>, Vec<usize>),
}

impl State {
   fn new(width: u32, height: u32) -> anyhow::Result<Self> {
      Ok(Self {
         depth: Surface::new(width, height),
         model: Self::load_model(include_str!("assets/suzanne.obj"))?,
      })
   }

   fn load_model(obj: &str) -> anyhow::Result<(Vec<Vertex>, Vec<usize>)> {
      let obj = obj::parse(obj)?;
      let model = &obj.objects[0];

      let mut vertices = Vec::new();
      for vertex in &model.vertices {
         vertices.push(Vertex::new(
            vertex.x as f32,
            vertex.y as f32,
            vertex.z as f32,
         ));
      }

      let mut indices = Vec::new();
      for geometry in &model.geometry {
         for shape in &geometry.shapes {
            if let Primitive::Triangle((a, ..), (b, ..), (c, ..)) = shape.primitive {
               indices.push(a);
               indices.push(b);
               indices.push(c);
            }
         }
      }
      Ok((vertices, indices))
   }

   fn draw(&mut self, color: &mut Surface<Rgba<u8>, &mut [Rgba<u8>]>, time: f64) {
      let model = Mat4::from_scale(Vec3::new(0.4, 0.4, 0.4));
      let model = model * Mat4::from_rotation_y(time as f32);
      let projection = Mat4::perspective_rh(
         deg_to_rad(75.0),
         color.width() as f32 / color.height() as f32,
         0.0001,
         1000.0,
      );

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

      ClearPipeline {
         surface: &mut self.depth,
         color: Depth(f32::INFINITY),
      }
      .run();

      TrianglePipeline {
         mesh: &(self.model.0.as_slice(), self.model.1.as_slice()),
         viewport: Viewport {
            x: 0,
            y: 0,
            width: color.width(),
            height: color.height(),
         },
         color_attachment: color,
         depth_attachment: &mut self.depth,
         depth_map: &Depth,
         vertex_shader: &|vertex| {
            let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
            let position = projection * model * position;
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
         fragment_shader: &|(r, g, b)| Rgba {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: 255,
         },
      }
      .run();
   }
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

   let mut state = State::new(width, height)?;

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
            state.draw(&mut surface, time);
            if let Err(error) = pixels.render() {
               eprintln!("error: {}", error);
            }
         }
         _ => (),
      }
   });
}
