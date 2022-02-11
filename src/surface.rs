//! A pixel surface.

use std::ops::{Deref, DerefMut};

use crate::pixelformat::Pixel;

#[derive(Clone)]
pub struct Surface<P, B = Vec<P>>
where
   P: Pixel,
   B: Deref<Target = [P]> + DerefMut<Target = [P]>,
{
   width: u32,
   height: u32,
   data: B,
}

impl<P> Surface<P>
where
   P: Pixel,
{
   pub fn new(width: u32, height: u32) -> Self {
      Self {
         width,
         height,
         data: vec![Default::default(); width as usize * height as usize],
      }
   }
}

impl<P, B> Surface<P, B>
where
   P: Pixel,
   B: Deref<Target = [P]> + DerefMut<Target = [P]>,
{
   pub fn from_buffer(width: u32, height: u32, buffer: B) -> Self {
      Self {
         width,
         height,
         data: buffer,
      }
   }

   pub fn width(&self) -> u32 {
      self.width
   }

   pub fn height(&self) -> u32 {
      self.height
   }

   pub fn data(&self) -> &[P] {
      &self.data
   }

   pub fn data_mut(&mut self) -> &mut [P] {
      &mut self.data
   }

   pub fn set(&mut self, x: u32, y: u32, pixel: P) {
      if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
         return;
      }
      self.data[x as usize + y as usize * self.width as usize] = pixel;
   }
}
