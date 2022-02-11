//! Various pixel formats.

use bytemuck::{Pod, Zeroable};

/// RGB pixels.
#[derive(Clone, Copy)]
pub struct Rgba<T>
where
   T: Pod,
{
   pub r: T,
   pub g: T,
   pub b: T,
   pub a: T,
}

impl<T> Default for Rgba<T>
where
   T: Pod + Default,
{
   fn default() -> Self {
      Self {
         r: Default::default(),
         g: Default::default(),
         b: Default::default(),
         a: Default::default(),
      }
   }
}

unsafe impl<T> Zeroable for Rgba<T>
where
   T: Pod,
{
   fn zeroed() -> Self {
      Self {
         r: Zeroable::zeroed(),
         g: Zeroable::zeroed(),
         b: Zeroable::zeroed(),
         a: Zeroable::zeroed(),
      }
   }
}

unsafe impl<T> Pod for Rgba<T> where T: Pod {}

/// Depth format.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Depth<T>(pub T);

impl<T> Default for Depth<T>
where
   T: Default,
{
   fn default() -> Self {
      Self(Default::default())
   }
}

/// A pixel format.
pub trait Pixel: Copy + Default {}

impl<T> Pixel for Rgba<T> where T: Copy + Default + Pod {}
impl<T> Pixel for Depth<T> where T: Copy + Default + Pod {}
