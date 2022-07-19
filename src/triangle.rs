use nalgebra as nl;
use nl::{matrix, vector, Matrix2x3, Matrix4, Matrix4x3, Vector3, Vector4};

#[derive(Clone, Copy)]
pub struct Triangle {
    pub points: Matrix4x3<f32>,
    pub texture_points: Matrix2x3<f32>,
    pub mid: Vector4<f32>,
    pub col: [u8; 3],
}

impl Triangle {
    pub fn new(points: Matrix4x3<f32>) -> Triangle {
        Triangle {
            points,
            texture_points: Matrix2x3::zeros(),
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn from_vercices(v1: [f32; 3], v2: [f32; 3], v3: [f32; 3]) -> Triangle {
        let points = matrix![v1[0], v2[0], v3[0];
                                v1[1], v2[1], v3[1];
                                v1[2], v2[2], v3[2];
                                1., 1., 1.;];
        Triangle {
            points,
            texture_points: Matrix2x3::zeros(),
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn from_vectors(v1: &Vector3<f32>, v2: &Vector3<f32>, v3: &Vector3<f32>) -> Triangle {
        let points = matrix![v1[0], v2[0], v3[0];
                                v1[1], v2[1], v3[1];
                                v1[2], v2[2], v3[2];
                                1., 1., 1.;];
        Triangle {
            points,
            texture_points: Matrix2x3::zeros(),
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn from_points(
        p1: f32,
        p2: f32,
        p3: f32,
        p4: f32,
        p5: f32,
        p6: f32,
        p7: f32,
        p8: f32,
        p9: f32,
    ) -> Triangle {
        let points = matrix! [p1, p4, p7;
                                 p2, p5, p8;
                                 p3, p6, p9;
                                 1., 1., 1.];
        Triangle {
            points,
            texture_points: Matrix2x3::zeros(),
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn point3(&self, index: usize) -> Vector3<f32> {
        vector![
            self.points[(0, index)],
            self.points[(1, index)],
            self.points[(2, index)]
        ]
    }

    pub fn normal(&self) -> Vector3<f32> {
        let line1 = vector![
            self.points[(0, 0)] - self.points[(0, 1)],
            self.points[(1, 0)] - self.points[(1, 1)],
            self.points[(2, 0)] - self.points[(2, 1)]
        ];
        let line2 = vector![
            self.points[(0, 0)] - self.points[(0, 2)],
            self.points[(1, 0)] - self.points[(1, 2)],
            self.points[(2, 0)] - self.points[(2, 2)]
        ];
        let nor = line1.cross(&line2);
        nor * 1. / nor.norm()
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.points += matrix![x, x, x;
                                   y, y, y;
                                   z, z, z;
                                   0., 0., 0.];
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        let x_rot = Matrix4::new_rotation(Vector3::x() * x);
        let y_rot = Matrix4::new_rotation(Vector3::y() * y);
        let z_rot = Matrix4::new_rotation(Vector3::z() * z);
        self.points = x_rot * y_rot * z_rot * self.points;
    }
}
