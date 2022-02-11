//! Surface clearing pipeline.

use std::ops::{Deref, DerefMut};

use crate::pixelformat::Pixel;
use crate::surface::Surface;

pub struct ClearPipeline<'a, Pix, Pbuf>
where
   Pix: Pixel,
   Pbuf: Deref<Target = [Pix]> + DerefMut,
{
   pub surface: &'a mut Surface<Pix, Pbuf>,
   pub color: Pix,
}

impl<'a, Pix, Pbuf> ClearPipeline<'a, Pix, Pbuf>
where
   Pix: Pixel,
   Pbuf: Deref<Target = [Pix]> + DerefMut,
{
   /// Runs the clear operation.
   pub fn run(&mut self) {
      self.surface.data_mut().iter_mut().for_each(|c| {
         *c = self.color;
      })
   }
}
