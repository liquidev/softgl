//! Support for getting vertices out of meshes.

/// A mesh of triangles.
pub trait Mesh {
   type Vertex;

   fn each_triangle(&self, f: impl FnMut([&Self::Vertex; 3]));
}

impl<'a, T> Mesh for &'a [T] {
   type Vertex = T;

   fn each_triangle(&self, mut f: impl FnMut([&Self::Vertex; 3])) {
      for triangle in self.chunks_exact(3) {
         f([&triangle[0], &triangle[1], &triangle[2]])
      }
   }
}

impl<'a, T> Mesh for (&'a [T], &'a [usize]) {
   type Vertex = T;

   fn each_triangle(&self, mut f: impl FnMut([&Self::Vertex; 3])) {
      for index in self.1.chunks_exact(3) {
         f([&self.0[index[0]], &self.0[index[1]], &self.0[index[2]]])
      }
   }
}
