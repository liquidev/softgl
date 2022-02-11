//! Support for values that vary from vertex to vertex.

use crate::math::Lerp;

pub trait Varying: std::fmt::Debug + Copy + 'static {
   fn vary(&self, other: Self, coeff: f32) -> Self;
}

impl Varying for () {
   fn vary(&self, _other: Self, _coeff: f32) -> Self {}
}

impl Varying for f32 {
   fn vary(&self, other: Self, coeff: f32) -> Self {
      self.lerp(other, coeff)
   }
}

macro_rules! impl_varying {
   ($($Ts:tt),+; $($nums:tt),+) => {
      impl<$($Ts,)+> Varying for ($($Ts,)+)
      where
         $($Ts: Varying,)+
      {
         fn vary(&self, other: Self, coeff: f32) -> Self {
            (
               $(self.$nums.vary(other.$nums, coeff),)+
            )
         }
      }
   }
}

impl_varying!(A, B; 0, 1);
impl_varying!(A, B, C; 0, 1, 2);
impl_varying!(A, B, C, D; 0, 1, 2, 3);
impl_varying!(A, B, C, D, E; 0, 1, 2, 3, 4);
impl_varying!(A, B, C, D, E, F; 0, 1, 2, 3, 4, 5);
impl_varying!(A, B, C, D, E, F, G; 0, 1, 2, 3, 4, 5, 6);
impl_varying!(A, B, C, D, E, F, G, H; 0, 1, 2, 3, 4, 5, 6, 7);
