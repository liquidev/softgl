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

   fn flat_index(&self, x: u32, y: u32) -> usize {
      x as usize + y as usize * self.width as usize
   }

   pub fn get(&self, x: u32, y: u32) -> Option<P> {
      let index = self.flat_index(x, y);
      if index >= self.data.len() {
         return None;
      }
      Some(unsafe { *self.data.get_unchecked(index) })
   }

   pub fn set(&mut self, x: u32, y: u32, pixel: P) {
      let index = self.flat_index(x, y);
      if index >= self.data.len() {
         return;
      }
      unsafe {
         *self.data.get_unchecked_mut(index) = pixel;
      }
   }
}
