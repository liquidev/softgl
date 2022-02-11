//! Math utilities such as `lerp`.

pub trait Lerp<F> {
   fn lerp(&self, other: Self, t: F) -> Self;
}

impl Lerp<f32> for f32 {
   fn lerp(&self, other: Self, t: f32) -> f32 {
      self + t * (other - self)
   }
}
