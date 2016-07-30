// Copyright © 2016 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std;

pub use std::f32::consts::PI as PI;

/// A 4x4 matrix.
pub struct Mat4(pub [[f32; 4]; 4]);

impl std::ops::Deref for Mat4 {
    type Target = [[f32; 4]; 4];

    fn deref(&self) -> &[[f32; 4]; 4] {
        &self.0
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self[k][j] * rhs[i][k];
                }
            }
        }
        Mat4(result)
    }
}

impl Mat4 {
    /// Returns a 4x4 identity matrix.
    pub fn identity() -> Self {
        Mat4([[1.0, 0.0, 0.0, 0.0],
              [0.0, 1.0, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [0.0, 0.0, 0.0, 1.0]])
    }

    /// Performs a rotation about the x-axis.
    pub fn rotation_x(theta: f32) -> Self {
        let s = theta.sin();
        let c = theta.cos();
        Mat4([[1.0, 0.0, 0.0, 0.0],
              [0.0,   c,   s, 0.0],
              [0.0,  -s,   c, 0.0],
              [0.0, 0.0, 0.0, 1.0]])
    }

    /// Performs a rotation about the y-axis.
    pub fn rotation_y(theta: f32) -> Self {
        let s = theta.sin();
        let c = theta.cos();
        Mat4([[  c, 0.0,   s, 0.0],
              [0.0, 1.0, 0.0, 0.0],
              [ -s, 0.0,   c, 0.0],
              [0.0, 0.0, 0.0, 1.0]])
    }

    /// Performs a rotation about the z-axis.
    pub fn rotation_z(theta: f32) -> Self {
        let s = theta.sin();
        let c = theta.cos();
        Mat4([[  c,   s, 0.0, 0.0],
              [ -s,   c, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Mat4([[1.0, 0.0, 0.0, 0.0],
              [0.0, 1.0, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [  x,   y,   z, 1.0]])
    }
}

/// A 3-component vector.
pub struct Vec3([f32; 3]);

impl Vec3 {
    /// Constructs a new Vec3 from its components.
    pub fn new(components: [f32; 3]) -> Vec3 {
        Vec3(components)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self[0], self[1], self[2])
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.0[i]
    }
}

impl std::convert::AsRef<[f32; 3]> for Vec3 {
    fn as_ref(&self) -> &[f32; 3] {
        &self.0
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3([self[0] * scalar, self[1] * scalar, self[2] * scalar])
    }
}

impl<'a> std::ops::Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3([self[0] * scalar, self[1] * scalar, self[2] * scalar])
    }
}

impl<'a> std::ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: &'a Vec3) -> Vec3 {
        vec * self
    }
}

impl<'a> std::ops::Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}

impl<'a, 'b> std::ops::Add<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}

impl<'a> std::ops::Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3([self[0] - other[0], self[1] - other[1], self[2] - other[2]])
    }
}

impl<'a, 'b> std::ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3([self[0] - other[0], self[1] - other[1], self[2] - other[2]])
    }
}

impl Vec3 {
    /// Calculates the dot product of this Vec3 and another.
    pub fn dot<V>(&self, other: V) -> f32 where V: AsRef<[f32; 3]> {
        let o = other.as_ref();
        self[0] * o[0] + self[1] * o[1] + self[2] * o[2]
    }
}