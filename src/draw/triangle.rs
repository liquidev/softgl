//! The rasterization pipeline.

use std::mem::swap;
use std::ops::{Deref, DerefMut};

use crate::math::Lerp;
use crate::mesh::Mesh;
use crate::pixelformat::Pixel;
use crate::surface::Surface;
use crate::varying::Varying;
use crate::vertex::{Position, ToDeviceCoordinates, Viewport};

pub struct TrianglePipeline<'a, M, Vert, Pix, Pbuf, Pos, Vs, Fs, Var>
where
   M: Mesh<Vertex = Vert>,
   Vert: Copy + 'static,
   Pix: Pixel,
   Pbuf: Deref<Target = [Pix]> + DerefMut,
   Position<Pos>: ToDeviceCoordinates,
   Vs: Fn(&Vert) -> (Position<Pos>, Var),
   Fs: Fn(Var) -> Pix,
   Var: Varying,
{
   pub mesh: M,
   pub viewport: Viewport,
   pub color_attachment: &'a mut Surface<Pix, Pbuf>,
   pub vertex_shader: Vs,
   pub fragment_shader: Fs,
}

impl<'a, M, Vert, Pix, Pbuf, Pos, Vs, Fs, Var>
   TrianglePipeline<'a, M, Vert, Pix, Pbuf, Pos, Vs, Fs, Var>
where
   M: Mesh<Vertex = Vert>,
   Vert: Copy + 'static,
   Pix: Pixel,
   Pbuf: Deref<Target = [Pix]> + DerefMut,
   Position<Pos>: ToDeviceCoordinates,
   Vs: Fn(&Vert) -> (Position<Pos>, Var),
   Fs: Fn(Var) -> Pix,
   Var: Varying,
{
   fn rasterize_triangle(vertices: &[(f32, f32, Var); 3], mut f: impl FnMut(i32, i32, Var)) {
      let edges = [
         Edge::new(vertices[0], vertices[1]),
         Edge::new(vertices[1], vertices[2]),
         Edge::new(vertices[2], vertices[0]),
      ];
      let long_edge =
         edges.iter().enumerate().max_by_key(|(_i, edge)| edge.y_length() as i32).unwrap().0;
      let short_edge_1: usize = [1, 2, 0][long_edge];
      let short_edge_2: usize = [2, 0, 1][long_edge];
      let (long_edge, short_edge_1, short_edge_2) =
         (edges[long_edge], edges[short_edge_1], edges[short_edge_2]);
      short_edge_1.each_scanline(&long_edge, |scanline| scanline.each_pixel(&mut f));
      short_edge_2.each_scanline(&long_edge, |scanline| scanline.each_pixel(&mut f));
   }

   /// Runs the triangle pipeline.
   pub fn run(&mut self) {
      self.mesh.each_triangle(|triangle| {
         let positions = [
            (self.vertex_shader)(triangle[0]),
            (self.vertex_shader)(triangle[1]),
            (self.vertex_shader)(triangle[2]),
         ];
         let vertices = [
            positions[0].0.to_device_coordinates(&self.viewport),
            positions[1].0.to_device_coordinates(&self.viewport),
            positions[2].0.to_device_coordinates(&self.viewport),
         ];
         let vertices = [
            (vertices[0].0 as f32, vertices[0].1 as f32, positions[0].1),
            (vertices[1].0 as f32, vertices[1].1 as f32, positions[1].1),
            (vertices[2].0 as f32, vertices[2].1 as f32, positions[2].1),
         ];
         Self::rasterize_triangle(&vertices, |x, y, varying| {
            let color = (self.fragment_shader)(varying);
            self.color_attachment.set(x as u32, y as u32, color)
         });
      })
   }
}

#[derive(Debug, Clone, Copy)]
struct Edge<Var>
where
   Var: Varying,
{
   x1: f32,
   y1: f32,
   var1: Var,
   x2: f32,
   y2: f32,
   var2: Var,
}

impl<Var> Edge<Var>
where
   Var: Varying,
{
   fn new(mut a: (f32, f32, Var), mut b: (f32, f32, Var)) -> Self {
      if a.1 > b.1 {
         swap(&mut a, &mut b);
      }
      Self {
         x1: a.0,
         y1: a.1,
         var1: a.2,
         x2: b.0,
         y2: b.1,
         var2: b.2,
      }
   }

   fn y_length(&self) -> f32 {
      self.y2 - self.y1
   }

   fn each_scanline(&self, long: &Edge<Var>, mut f: impl FnMut(Scanline<Var>)) {
      for y in (self.y1 as i32)..(self.y2 as i32) {
         let short_t = (y as f32 - self.y1) / (self.y2 - self.y1);
         let long_t = (y as f32 - long.y1) / (long.y2 - long.y1);
         let short_x = self.x1.lerp(self.x2, short_t);
         let long_x = long.x1.lerp(long.x2, long_t);
         let short_var = self.var1.vary(self.var2, short_t);
         let long_var = long.var1.vary(long.var2, long_t);
         f(Scanline::new(
            short_x, short_var, long_x, long_var, y as f32,
         ))
      }
   }
}

#[derive(Debug, Clone, Copy)]
struct Scanline<Var>
where
   Var: Varying,
{
   x1: f32,
   var1: Var,
   x2: f32,
   var2: Var,
   y: f32,
}

impl<Var> Scanline<Var>
where
   Var: Varying,
{
   fn new(mut x1: f32, mut var1: Var, mut x2: f32, mut var2: Var, y: f32) -> Self {
      if x1 > x2 {
         (x1, var1, x2, var2) = (x2, var2, x1, var1);
      }
      Self {
         x1,
         var1,
         x2,
         var2,
         y,
      }
   }

   fn each_pixel(&self, mut f: impl FnMut(i32, i32, Var)) {
      for x in (self.x1 as i32)..(self.x2 as i32) {
         let t = (x as f32 - self.x1) / (self.x2 - self.x1);
         let var = self.var1.vary(self.var2, t);
         f(x, self.y as i32, var)
      }
   }
}
