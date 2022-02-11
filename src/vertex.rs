//! Vertex format.

/// Position data.
#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
   pub x: T,
   pub y: T,
   pub z: T,
   pub w: T,
}

/// A render target viewport.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
   pub x: i32,
   pub y: i32,
   pub width: u32,
   pub height: u32,
}

pub trait ToDeviceCoordinates {
   /// Converts the position to device coordinates, given the provided viewport.
   fn to_device_coordinates(&self, viewport: &Viewport) -> (i32, i32, f32);
}

/// Convert floating point positions to device coordinates.
impl ToDeviceCoordinates for Position<f32> {
   fn to_device_coordinates(&self, viewport: &Viewport) -> (i32, i32, f32) {
      let (x, y) = (self.x, self.y);
      let x = ((x + 1.0) * 0.5) * viewport.width as f32;
      let y = ((-y + 1.0) * 0.5) * viewport.height as f32;
      (x as i32 + viewport.x, y as i32 + viewport.y, self.z)
   }
}
