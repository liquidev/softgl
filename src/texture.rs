//! Textures and samplers.

use std::ops::{Deref, DerefMut};

use bytemuck::Zeroable;

use crate::pixelformat::Pixel;
use crate::surface::Surface;

pub struct Sampler<'a, P, B>
where
   P: Pixel,
   B: Deref<Target = [P]> + DerefMut,
{
   texture: &'a Surface<P, B>,
}

impl<'a, P, B> Sampler<'a, P, B>
where
   P: Pixel + Zeroable,
   B: Deref<Target = [P]> + DerefMut,
{
   pub fn new(texture: &'a Surface<P, B>) -> Self {
      Self { texture }
   }

   pub fn sample(&self, x: f32, y: f32) -> P {
      let x = (x * self.texture.width() as f32) as u32;
      let y = (y * self.texture.height() as f32) as u32;
      self.texture.get(x, y).unwrap_or_else(|| P::zeroed())
   }
}
